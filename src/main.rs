use crossterm::event::{KeyCode, KeyModifiers};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
};

mod app;
mod components;
mod events;
mod monday;
mod objects;
mod queries;
mod utils;
mod views;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Terminal
    let mut terminal = app::start_terminal();
    //Receiver Channel
    let rx = events::start_input_handling();
    //Menu
    let mut app = app::App::new();

    loop {
        //Draw frame
        terminal.draw(|mut rect| match app.active_menu_item {
            views::MenuItem::Home => views::Home::render(&mut rect, &app),
            views::MenuItem::Boards => views::BoardList::render(&mut rect, &app),
            views::MenuItem::Items => views::ItemList::render(&mut rect, &app),
            views::MenuItem::ItemDetail => views::ItemDetail::render(&mut rect, &app),
            views::MenuItem::ItemOptions => views::ItemOptions::render(&mut rect, &mut app), 
        })?;

        //Deal with input
        match rx.recv()? {
            events::Event::Input(event) => {
                //Quit
                if event.code == KeyCode::Esc {
                    app::stop_terminal(&mut terminal);
                    break;
                }

                // Key Input
                events::handle_key_input(event, &mut app);

                //View events
                match app.active_menu_item {
                    views::MenuItem::Home => views::Home.process_input_event(event, &mut app),
                    views::MenuItem::Boards => {
                        views::BoardList.process_input_event(event, &mut app)
                    }
                    views::MenuItem::Items => views::ItemList.process_input_event(event, &mut app),
                    views::MenuItem::ItemDetail => {
                        views::ItemDetail.process_input_event(event, &mut app)
                    }, 
                    views::MenuItem::ItemOptions => views::ItemOptions.process_input_event(event, &mut app), 
                    _ => {}
                }
            }
            events::Event::Tick => {}
        }
    }
    Ok(())
}
