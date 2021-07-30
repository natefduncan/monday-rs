use super::objects;

pub fn search_boards(query: String, boards: &Vec<objects::Board>) -> Vec<objects::Board> {
    let mut output: Vec<objects::Board> = Vec::new();
    let query_lower: String = query.to_lowercase();

    for board in boards.clone() {
        if board
            .name
            .to_lowercase()
            .split_whitespace()
            .any(|x| x.contains(&query_lower))
        {
            output.push(board.clone());
        }
    }
    return output;
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