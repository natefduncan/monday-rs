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