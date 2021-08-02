use super::objects;
use aho_corasick::AhoCorasickBuilder;

fn search(query : String, vector : &Vec<String>) -> Vec<bool> {
    let patterns = query.split_whitespace(); 
    let ac = AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .build(patterns);
    vector.iter().map(|string| {
        match ac.find(string).map(|m| m.pattern()) {
            Some(v) => true, 
            None => false
        }
    }).collect::<Vec<bool>>()
}

fn filter_by_bool<T : Clone>(filter_vec : &Vec<T>, bool_vec : &Vec<bool>) -> Vec<T> {
    let mut output : Vec<T> = Vec::new(); 
    let zip = filter_vec.iter().zip(bool_vec.iter()); 
    for (i, val) in filter_vec.iter().enumerate() {
        if bool_vec[i] {
            output.push(val.clone()); 
        }
    }
    return output; 
}

pub fn search_boards(query: String, boards: &Vec<objects::Board>) -> Vec<objects::Board> {
    let query_lower: String = query.to_lowercase();
    let vec_string : Vec<String> = boards.iter().map(|board| board.name.clone()).collect::<Vec<String>>(); 
    let search_bool = search(query, &vec_string); 
    let filtered = filter_by_bool::<objects::Board>(&boards, &search_bool); 
    return filtered
}

pub fn search_items(query: String, items: &Vec<objects::Item>) -> Vec<objects::Item> {
    let query_lower: String = query.to_lowercase();
    let vec_string : Vec<String> = items.iter().map(|item| item.name.clone()).collect::<Vec<String>>(); 
    let search_bool = search(query, &vec_string); 
    let filtered = filter_by_bool::<objects::Item>(&items, &search_bool); 
    return filtered
} 

pub fn filter_boards(boards : &Vec<objects::Board>, search : &Vec<char>) -> Vec<objects::Board> {
    let output : Vec<objects::Board>; 
    //Filter by search element
    if search.len() > 0 {
        let search_string: String =
            search.iter().map(|c| c.to_string()).collect::<String>();
        output = search_boards(search_string, boards);
    } else {
        output = boards.clone();
    }
    return output; 
}


pub fn filter_items(items : &Vec<objects::Item>, search : &Vec<char>) -> Vec<objects::Item> {
    let output : Vec<objects::Item>; 
    //Filter by search element
    if search.len() > 0 {
        let search_string: String =
            search.iter().map(|c| c.to_string()).collect::<String>();
        output = search_items(search_string, items);
    } else {
        output = items.clone();
    }
    return output; 
}