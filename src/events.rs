use crossterm::event::{self, Event as CEvent, KeyEvent, KeyCode, KeyModifiers};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    Terminal, 
    backend::CrosstermBackend
}; 
use std::io; 
use super::app; 
use super::views; 

//Event loop enum
pub enum Event<I> {
    Input(I),
    Tick,
}

pub fn start_input_handling() -> mpsc::Receiver<Event<KeyEvent>> {
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
    return rx;
}

pub fn handle_menu(event : KeyEvent, app : &mut app::App, terminal : &mut Terminal<CrosstermBackend<io::Stdout>>) {
    match event.modifiers {
        KeyModifiers::SHIFT => {
            match event.code {
                KeyCode::Char('H') => app.active_menu_item = views::MenuItem::Home, 
                KeyCode::Char('B') => app.active_menu_item = views::MenuItem::Boards, 
                KeyCode::Char('I') => app.active_menu_item = views::MenuItem::Items, 
                KeyCode::Char('Q') => {
                    app::stop_terminal(terminal);
                }, 
                _ => {}
            }
        }, 
        _ => {}
            
        
    }
}

pub fn handle_key_input(event : KeyEvent, app : &mut app::App) {
    match event.code {
        KeyCode::Char(c) => app.key_input.push(c),
        _ => {}
    }
}