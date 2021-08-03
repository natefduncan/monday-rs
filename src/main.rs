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
    //APP
    //Terminal
    let mut terminal = app::start_terminal();
    //Receiver Channel
    let rx = events::start_input_handling();
    //Menu
    let mut app = app::App::new();

    loop {
        terminal.draw(|mut rect| match app.active_menu_item {
            views::MenuItem::Home => views::Home::render(&mut rect, &app),
            views::MenuItem::Boards => views::BoardList::render(&mut rect, &app),
            views::MenuItem::Items => views::ItemList::render(&mut rect, &app),
            views::MenuItem::ItemDetail => views::ItemDetail::render(&mut rect, &app),
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
