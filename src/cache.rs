use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use super::app; 

// To store status column
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardMeta {
    pub id: String,
    pub status_column_id: String,
    pub user_column_id: String,
}

impl BoardMeta {
    pub fn new() -> BoardMeta {
        BoardMeta {
            id: String::from(""),
            status_column_id: String::from(""),
            user_column_id : String::from("")
        } 
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub boards: Vec<BoardMeta>,
    pub app_state : app::AppState
}

impl Cache {
    pub fn new() -> Cache {
        if exists() {
            return read();
        } else {
            create().expect("could not create .monday dir");
            let cache = Cache { boards: vec![], app_state : app::AppState::new()};
            write(&cache.clone()).expect("could not write");
            return cache;
        }
    }
    
    pub fn update_board_meta(&mut self, board_meta : BoardMeta) {
        if self.boards.iter().filter(|board| board.id == board_meta.id).cloned().collect::<Vec<BoardMeta>>().len() == 0 {
            self.boards.push(board_meta); 
        } else {
            self.boards = self.boards.iter().filter(|board| board.id != board_meta.id).cloned().collect::<Vec<BoardMeta>>(); 
            self.boards.push(board_meta); 
        }
    }

    pub fn board_has_meta(&self, board_id : String) -> bool {
        self.boards.iter().filter(|board| board.id == board_id).cloned().collect::<Vec<BoardMeta>>().len() > 0
    }

    pub fn get_board_meta(&self, board_id : String) -> BoardMeta { 
        let board_meta = self.boards.iter().filter(|board| board.id == board_id).cloned().collect::<Vec<BoardMeta>>(); 
        board_meta
        .into_iter()
        .nth(0)
        .unwrap_or(BoardMeta::new())
    }
}

pub fn exists() -> bool {
    let file_path = get_cache_path();
    std::path::Path::new(&file_path).exists()
}

pub fn create() -> Result<(), std::io::Error> {
    let file_path = get_cache_dir();
    std::fs::create_dir_all(file_path)?;
    Ok(())
}

pub fn read() -> Cache {
    let file_path = get_cache_path();
    let data = std::fs::read_to_string(file_path).expect("Unable to read file");
    let res: Cache = serde_json::from_str(&data).expect("Unable to parse");
    return res;
}

pub fn write(cache: &Cache) -> Result<(), std::io::Error> {
    let file_path = get_cache_path();
    if exists() {
        std::fs::remove_file(file_path.clone()).expect("could not remove file");
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(file_path)?;
    serde_json::to_writer(file, &cache).expect("could not write to file");
    Ok(())
}

pub fn get_cache_path() -> String {
    let home_dir = home_dir().unwrap().into_os_string().into_string().unwrap();
    let file_path = format!("{}/{}", home_dir, ".monday/cache.json");
    return file_path;
}

pub fn get_cache_dir() -> String {
    let home_dir = home_dir().unwrap().into_os_string().into_string().unwrap();
    let file_path = format!("{}/{}", home_dir, ".monday");
    return file_path;
}
