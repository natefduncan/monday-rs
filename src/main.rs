use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
};

use crossterm::event::{KeyCode, KeyModifiers};

mod app;
mod events;
mod monday;
mod objects;
mod queries;
mod utils;
mod views;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //APP
    //Terminal
    let mut terminal = app::start_terminal();
    //Receiver Channel
    let rx = events::start_input_handling();
    //Menu
    let menu_titles = vec!["Home", "Boards", "Items", "Quit"];
    let mut app = app::App::new();

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

            let search_text: String = app.search.iter().map(|x| x.to_string()).collect::<String>();
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
                .select(app.active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            match app.active_menu_item {
                views::MenuItem::Home => rect.render_widget(views::Home::render(), chunks[1]),
                views::MenuItem::Boards => {
                    let board_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(40), Constraint::Percentage(60)].as_ref(),
                        )
                        .split(chunks[1]);
                    let board_filtered = utils::filter_boards(&app.boards, &app.search);
                    let (left, right) = views::BoardList::render(&board_filtered, &app.list_state);
                    rect.render_stateful_widget(left, board_chunks[0], &mut app.list_state);
                    rect.render_widget(right, board_chunks[1]);
                }
                views::MenuItem::Items => {
                    let filtered = utils::filter_items(&app.items, &app.search);
                    let list_items = views::ItemList::render(&filtered, &app.list_state);
                    rect.render_stateful_widget(list_items, chunks[1], &mut app.list_state);
                }, 
                views::MenuItem::ItemDetail => {
                    let detail = views::ItemDetail::render(&app.item_detail); 
                    rect.render_widget(detail, chunks[1]); 
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
                    KeyCode::Char('H') => app.active_menu_item = views::MenuItem::Home,
                    KeyCode::Char('B') => app.active_menu_item = views::MenuItem::Boards,
                    KeyCode::Char('I') => app.active_menu_item = views::MenuItem::Items,
                    _ => {}
                },
                _ => match event.code {
                    KeyCode::Down => match app.active_menu_item {
                        views::MenuItem::Boards => views::BoardList::keydown(&mut app),
                        views::MenuItem::Items => views::ItemList::keydown(&mut app),
                        _ => (),
                    },
                    KeyCode::Up => match app.active_menu_item {
                        views::MenuItem::Boards => views::BoardList::keyup(&mut app),
                        views::MenuItem::Items => views::ItemList::keyup(&mut app),
                        _ => (),
                    },
                    KeyCode::Backspace => {
                        app.search.pop();
                    }
                    KeyCode::Enter => match app.active_menu_item {
                        views::MenuItem::Boards => views::BoardList::keyenter(&mut app),
                        views::MenuItem::Items => views::ItemList::keyenter(&mut app), 
                        _ => (),
                    },
                    KeyCode::Char(c) => {
                        app.search.push(c);
                        app.list_state.select(Some(0));
                    }
                    _ => {}
                },
            },
            events::Event::Tick => {}
        }
    }

    Ok(())
}
