use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;

// To store status column
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardMeta {
    pub id: String,
    pub status_column_id: String,
}

impl BoardMeta {
    pub fn new() -> BoardMeta {
        BoardMeta {
            id: String::from(""),
            status_column_id: String::from(""),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub boards: Vec<BoardMeta>,
}

impl Cache {
    pub fn new() -> Cache {
        if exists() {
            return read();
        } else {
            create().expect("could not create .monday dir");
            let cache = Cache { boards: vec![] };
            write(&cache.clone()).expect("could not write");
            return cache;
        }
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
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
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
