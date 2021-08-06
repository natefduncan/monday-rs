use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
    event::{KeyCode}, 
};
use reqwest;
use std::io;
use tui::{backend::CrosstermBackend, widgets::ListState, Terminal};
use serde::{Serialize, Deserialize}; 

use super::monday;
use super::objects;
use super::queries;
use super::views;
use super::cache; 

#[derive(Debug, Clone)]
pub struct App {
    pub list_state: ListState,
    pub boards: Vec<objects::Board>,
    pub board_detail : objects::Board, 
    pub groups : Vec<objects::Group>, 
    pub group_detail : objects::Group, 
    pub items: Vec<objects::Item>,
    pub item_detail: objects::Item,
    pub active_menu_item: views::MenuItem,
    pub key_input: Vec<char>,
    pub client: reqwest::blocking::Client,
    pub menu_titles: Vec<String>,
    pub status_labels: Vec<objects::Label>, 
    pub users : Vec<objects::User>, 
    pub cache : cache::Cache, 
    pub f : KeyCode, 
    pub current_user : objects::User, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub boards: Vec<objects::Board>,
    pub board_detail : objects::Board, 
    pub groups : Vec<objects::Group>, 
    pub group_detail : objects::Group, 
    pub items: Vec<objects::Item>,
    pub item_detail: objects::Item,
    pub active_menu_item: views::MenuItem,
    pub key_input: Vec<char>,
    pub menu_titles: Vec<String>,
    pub status_labels: Vec<objects::Label>, 
    pub users : Vec<objects::User>, 
    pub current_user : objects::User, 
}

impl From<App> for AppState {
    fn from(app: App) -> AppState {
        AppState {
            boards: app.boards.clone(),
            board_detail : app.board_detail.clone(), 
            groups : app.groups.clone(), 
            group_detail : app.group_detail.clone(), 
            items: app.items.clone(),
            item_detail: app.item_detail.clone(),
            active_menu_item: app.active_menu_item.clone(),
            key_input: app.key_input.clone(),
            menu_titles: app.menu_titles.clone(),
            status_labels: app.status_labels.clone(), 
            users : app.users.clone(), 
            current_user : app.current_user.clone()
        }
    }
}

impl From<AppState> for App {
    fn from(app_state: AppState) -> App {
        let cache = cache::Cache::new(); 
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let client = monday::get_client().expect("Could not get client.");
        let f = KeyCode::Null; 

        let app = App {
            boards : app_state.boards.clone(),
            board_detail : app_state.board_detail.clone(), 
            groups : app_state.groups.clone(),
            group_detail : app_state.group_detail.clone(), 
            items : app_state.items.clone(),
            item_detail : app_state.item_detail.clone(),
            active_menu_item : app_state.active_menu_item.clone(),
            key_input : app_state.key_input.clone(),
            menu_titles : app_state.menu_titles.clone(),
            status_labels : app_state.status_labels.clone(), 
            users : app_state.users.clone(),
            current_user : app_state.current_user.clone(),
            f : f, 
            client : client, 
            list_state : list_state, 
            cache : cache
        };
        return app; 
    }
}

impl App {
    pub fn new() -> App {
        let cache = cache::Cache::new(); 
        App::from(cache.app_state)
    }
}

impl AppState {
    pub fn new() -> AppState {
        let active_menu_item = views::MenuItem::Home;
        let key_input: Vec<char> = Vec::new();
        let client = monday::get_client().expect("Could not get client.");
        let boards: Vec<objects::Board> = queries::board_list(&client);
        let groups : Vec<objects::Group> = Vec::new(); 
        let items: Vec<objects::Item> = Vec::new();
 
        AppState {
            boards: boards,
            board_detail : objects::Board::new(), 
            groups : groups, 
            group_detail : objects::Group::new(), 
            items: items,
            item_detail: objects::Item::new(),
            active_menu_item: active_menu_item,
            key_input: key_input,
            menu_titles: vec!["Home", "Boards", "Groups", "Items", "Item Detail"]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
            status_labels : Vec::new(), 
            users : Vec::new(),
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
