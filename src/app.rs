use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
    event::{KeyCode}, 
};
use reqwest;
use std::io;
use tui::{backend::CrosstermBackend, widgets::ListState, Terminal};

use super::monday;
use super::objects;
use super::queries;
use super::views;
use super::cache; 

#[derive(Debug, Clone)]
pub struct App {
    pub list_state: ListState,
    pub boards: Vec<objects::Board>,
    pub items: Vec<objects::Item>,
    pub item_detail: objects::Item,
    pub active_menu_item: views::MenuItem,
    pub key_input: Vec<char>,
    pub client: reqwest::blocking::Client,
    pub menu_titles: Vec<String>,
    pub status_labels: Vec<objects::Label>, 
    pub cache : cache::Cache, 
    pub f : KeyCode, 
    pub current_user : objects::User, 
}


impl App {
    pub fn new() -> App {
        let active_menu_item = views::MenuItem::Home;
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let key_input: Vec<char> = Vec::new();
        let client = monday::get_client().expect("Could not get client.");
        let boards: Vec<objects::Board> = queries::board_list(&client);
        let items: Vec<objects::Item> = Vec::new();
 
        App {
            list_state: list_state,
            boards: boards,
            items: items,
            item_detail: objects::Item::new(),
            active_menu_item: active_menu_item,
            key_input: key_input,
            client: client.clone(),
            menu_titles: vec!["Home", "Boards", "Items", "Item Detail"]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            status_labels : Vec::new(), 
            cache : cache::Cache::new(), 
            f : KeyCode::Null,
            current_user : queries::current_user(&client.clone())
        }
    }
}

pub fn start_terminal() -> Terminal<CrosstermBackend<io::Stdout>> {
    enable_raw_mode().expect("start raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).expect("create alternate screen");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("create new terminal");
    terminal.clear().expect("clear");
    return terminal;
}

pub fn stop_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) {
    disable_raw_mode().expect("stop raw mode");
    terminal.clear().unwrap();
    terminal.show_cursor().expect("show cursor");
}
