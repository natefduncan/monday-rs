use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, ListState, Paragraph, Tabs,
    },
};

use crossterm::{
    event::{KeyCode, KeyModifiers},
};

mod monday;
mod objects;
mod queries;
mod views;
mod utils; 
mod events;
mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //APP
    //Terminal
    let mut terminal = app::start_terminal(); 
    //Receiver Channel
    let rx = events::start_input_handling(); 
    //Menu 
    let menu_titles = vec!["Home", "Boards", "Quit"];
    let mut active_menu_item = views::MenuItem::Boards;
    let mut list_state = ListState::default();
    list_state.select(Some(0));
    //Search 
    let mut search : Vec<char> = Vec::new(); 
    //Monday Data
    let client = monday::get_client().expect("Could not get client.");
    let mut boards : Vec<objects::Board> = queries::board_list(&client);
    let mut items : Vec<objects::Item> = Vec::new(); 

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
                views::MenuItem::Home => rect.render_widget(views::Home::render(), chunks[1]),
                views::MenuItem::Boards => {
                    let board_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(40), Constraint::Percentage(60)].as_ref(),
                        )
                        .split(chunks[1]);
                    let board_filtered = utils::filter_boards(&boards, &search); 
                    let (left, right) = views::BoardList::render(&board_filtered, &list_state);
                    rect.render_stateful_widget(left, board_chunks[0], &mut list_state);
                    rect.render_widget(right, board_chunks[1]);
                }, 
                views::MenuItem::Detail => {
                    let filtered = utils::filter_items(&items, &search); 
                    let list_items = views::BoardDetail::render(&filtered, &list_state);
                    rect.render_stateful_widget(list_items, chunks[1], &mut list_state);
                }
            }
            rect.render_widget(search_block, chunks[2]);
        })?;

        match rx.recv()? {
            events::Event::Input(event) => match event.modifiers {
                KeyModifiers::SHIFT => match event.code {
                    KeyCode::Char('Q') => {
                        app::stop_terminal(&mut terminal); 
                        break;
                    }
                    KeyCode::Char('H') => active_menu_item = views::MenuItem::Home,
                    KeyCode::Char('B') => active_menu_item = views::MenuItem::Boards,
                    _ => {}
                },
                _ => match event.code {
                    KeyCode::Down => {
                        if let Some(selected) = list_state.selected() {
                            let amount_boards = boards.len();
                            if selected >= amount_boards - 1 {
                                list_state.select(Some(0));
                            } else {
                                list_state.select(Some(selected + 1));
                            }
                        }
                    }
                    KeyCode::Up => {
                        if let Some(selected) = list_state.selected() {
                            let amount_boards = boards.len();
                            if selected > 0 {
                                list_state.select(Some(selected - 1)); 
                            } else {
                                list_state.select(Some(amount_boards - 1));
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        search.pop();
                    }, 
                    KeyCode::Enter => { 
                        let board_filtered = utils::filter_boards(&boards, &search); 
                        active_menu_item = views::MenuItem::Detail; 
                        let selected_board = board_filtered.get(list_state.selected().unwrap()).unwrap().clone(); 
                        items = queries::board_detail(&client, selected_board.id); 
                        search = Vec::new(); 
                    }
                    KeyCode::Char(c) => {
                        search.push(c);
                        list_state.select(Some(0)); 
                    }
                    _ => {}
                },
            },
            events::Event::Tick => {}
        }
    }

    Ok(())
}
