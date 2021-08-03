use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    terminal::{Frame}, 
    backend::{CrosstermBackend}
};
use std::io; 
use crossterm::event::{KeyCode, KeyModifiers};

mod app;
mod events;
mod monday;
mod objects;
mod queries;
mod utils;
mod views;
mod components; 


fn main() -> Result<(), Box<dyn std::error::Error>> {
    //APP
    //Terminal
    let mut terminal = app::start_terminal();
    //Receiver Channel
    let rx = events::start_input_handling();
    //Menu
    let mut app = app::App::new();

    loop {
        terminal.draw(|rect| {
            let chunks = components::get_default_chunks(&rect); 
            let search_block = components::get_search_block(&app); 
            let menu_block = components::get_menu_block(&app); 

            rect.render_widget(menu_block, chunks[0]);
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
                    rect.render_stateful_widget(left, board_chunks[0], &mut app.list_state.clone());
                    rect.render_widget(right, board_chunks[1]);
                }
                views::MenuItem::Items => {
                    let filtered = utils::filter_items(&app.items, &app.search);
                    let list_items = views::ItemList::render(&filtered, &app.list_state);
                    rect.render_stateful_widget(list_items, chunks[1], &mut app.list_state.clone());
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
