use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen},
};
use tui::{
    Terminal, 
    backend::{CrosstermBackend},
};  
use std::io; 

pub fn start_terminal() -> Terminal<CrosstermBackend<io::Stdout>> {
    enable_raw_mode().expect("start raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).expect("create alternate screen");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("create new terminal");
    terminal.clear().expect("clear"); 
    return terminal; 
}

pub fn stop_terminal(terminal : &mut Terminal<CrosstermBackend<io::Stdout>>) {
    disable_raw_mode().expect("stop raw mode");
    terminal.show_cursor().expect("show cursor");
}
