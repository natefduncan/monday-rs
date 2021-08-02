use super::objects;
use aho_corasick::AhoCorasickBuilder;
use std::fmt;

fn search(query : String, vector : &Vec<String>) -> Vec<usize> {
    let patterns = query.split_whitespace(); 
    let ac = AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .build(patterns);
    vector.iter().map(|string| {
        let matches = ac.find_iter(string).map(|m| m.pattern()).collect::<Vec<usize>>(); 
        matches.len()
    }).collect::<Vec<usize>>()
}

#[derive(Debug, Copy, Clone)]
struct Match<T> {
    obj : T, 
    count : usize
}

fn filter_by_matches<T : Clone + std::fmt::Debug >(filter_vec : &Vec<T>, match_vec : &Vec<usize>) -> Vec<T> {
    let mut output : Vec<T> = Vec::new(); 
    let zip = filter_vec.iter().zip(match_vec.iter()); 
    let mut matches : Vec<Match<T>> = zip.map(|(x,m)| Match { obj : x.clone(), count : *m}).collect::<Vec<Match<T>>>();
    matches.sort_by(|a, b| b.count.cmp(&a.count)); 
    return matches.iter().filter(|m| m.count > 0).map(|m| m.obj.clone()).collect::<Vec<T>>(); 
}

pub fn search_boards(query: String, boards: &Vec<objects::Board>) -> Vec<objects::Board> {
    let query_lower: String = query.to_lowercase();
    let vec_string : Vec<String> = boards.iter().map(|board| board.name.clone()).collect::<Vec<String>>(); 
    let search_bool = search(query, &vec_string); 
    let filtered = filter_by_matches::<objects::Board>(&boards, &search_bool); 
    return filtered
}

pub fn search_items(query: String, items: &Vec<objects::Item>) -> Vec<objects::Item> {
    let query_lower: String = query.to_lowercase();
    let vec_string : Vec<String> = items.iter().map(|item| item.name.clone()).collect::<Vec<String>>(); 
    let search_bool = search(query, &vec_string); 
    let filtered = filter_by_matches::<objects::Item>(&items, &search_bool); 
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