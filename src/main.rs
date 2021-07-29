use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, ListState, Paragraph, Tabs,
    },
    Terminal,
};

use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

mod monday;
mod objects;
mod queries;
mod views;

//Event loop enum
enum Event<I> {
    Input(I),
    Tick,
}

fn search_boards(query: String, boards: &Vec<objects::Board>, n: usize) -> Vec<objects::Board> {
    let mut output: Vec<objects::Board> = Vec::new();
    let query_lower: String = query.to_lowercase();

    for board in boards.clone() {
        if board
            .name
            .to_lowercase()
            .split_whitespace()
            .any(|x| x.contains(&query_lower))
        {
            output.push(board.clone());
        }
    }
    return output;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Fetch boards
    let client = monday::get_client().expect("Could not get client.");
    let mut board_vec: Vec<objects::Board> = queries::board_list(&client);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup input handling
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    let menu_titles = vec!["Home", "Boards", "Quit"];
    let mut active_menu_item = views::MenuItem::Boards;
    let mut board_list_state = ListState::default();
    let mut search: Vec<char> = Vec::new();

    board_list_state.select(Some(0));

    terminal.clear()?;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let search_text: String = search.iter().map(|x| x.to_string()).collect::<String>();
            let search_block = Paragraph::new(search_text)
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Search")
                        .border_type(BorderType::Plain),
                );

            let menu = menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                views::MenuItem::Home => rect.render_widget(views::render_home(), chunks[1]),
                views::MenuItem::Boards => {
                    let board_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(40), Constraint::Percentage(60)].as_ref(),
                        )
                        .split(chunks[1]);
                    let board_temp: Vec<objects::Board>;
                    if search.len() > 0 {
                        let search_string: String =
                            search.iter().map(|c| c.to_string()).collect::<String>();
                        board_temp = search_boards(search_string, &mut board_vec, 5);
                    } else {
                        board_temp = board_vec.clone();
                    }
                    let (left, right) = views::render_boards(&board_temp, &board_list_state);
                    rect.render_stateful_widget(left, board_chunks[0], &mut board_list_state);
                    rect.render_widget(right, board_chunks[1]);
                }
            }
            rect.render_widget(search_block, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.modifiers {
                KeyModifiers::SHIFT => match event.code {
                    KeyCode::Char('Q') => {
                        disable_raw_mode()?;
                        terminal.show_cursor()?;
                        break;
                    }
                    KeyCode::Char('H') => active_menu_item = views::MenuItem::Home,
                    KeyCode::Char('B') => active_menu_item = views::MenuItem::Boards,
                    _ => {}
                },
                _ => match event.code {
                    KeyCode::Down => {
                        if let Some(selected) = board_list_state.selected() {
                            let amount_boards = board_vec.len();
                            if selected >= amount_boards - 1 {
                                board_list_state.select(Some(0));
                            } else {
                                board_list_state.select(Some(selected + 1));
                            }
                        }
                    }
                    KeyCode::Up => {
                        if let Some(selected) = board_list_state.selected() {
                            let amount_boards = board_vec.len();
                            if selected > 0 {
                                board_list_state.select(Some(selected - 1));
                            } else {
                                board_list_state.select(Some(amount_boards - 1));
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        search.pop();
                    }
                    KeyCode::Char(c) => {
                        search.push(c);
                    }
                    _ => {}
                },
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
