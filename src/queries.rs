use super::monday;
use super::objects::*;
use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;

//BOARD LIST
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/board_list.graphql",
    response_derives = "Debug"
)]
struct BoardList;

pub fn board_list(client: &Client) -> Vec<Board> {
    let variables = board_list::Variables {
        limit : Some(50)
    };
    let res: Response<board_list::ResponseData> =
        monday::query::<BoardList>(&client, variables).expect("Could not execute query.");
    let boards = parse_board_list_response(res); 
    return boards;
}

fn parse_board_list_response(res : Response<board_list::ResponseData>) -> Vec<Board> {
    let data = res.data.expect("missing response data.");
    let boards: Vec<Board> = match data.boards {
        Some(arr) => arr
            .iter()
            .map(|board| match board {
                Some(b) => Board {
                    name: b.name.to_owned(),
                    id: b.id.to_owned(),
                },
                None => Board {
                    name: "No Name".to_string(),
                    id: "No ID".to_string(),
                },
            })
            .collect(),
        None => vec![],
    };
    return boards; 
}

//BOARD DETAIL
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/board_items.graphql",
    response_derives = "Debug,Clone"
)]
struct BoardDetail;

pub fn board_detail(client: &Client, board_id : String) -> Vec<Item> {
    let variables = board_detail::Variables {
        board_id : Some(board_id.parse::<i64>().expect("can convert to i64")), 
        limit : Some(50), 
        newest_first : Some(true), 
        page : Some(1)
    };
    let res: Response<board_detail::ResponseData> =
        monday::query::<BoardDetail>(&client, variables).expect("Could not execute query.");
    parse_board_detail_response(res)
}

fn parse_board_detail_response(res : Response<board_detail::ResponseData>) -> Vec<Item> {
    let data = res.data.expect("missing response data.");
    let board = data.boards.unwrap().into_iter().nth(0).expect("missing first value").unwrap();  
    //ITEMS
    let items = board.items.unwrap().iter().map(|item| {
        let i = item.clone().unwrap(); 
        Item {
            name : i.name, 
            //COLUMN VALUES
            column_values : i.column_values.unwrap().iter().map(|column_value| {
                let column_value = column_value.clone().unwrap();
                ColumnValue {
                    id : column_value.id, 
                    text : column_value.text.unwrap_or("No text".to_string()), 
                    title : column_value.title, 
                    type_ : column_value.type_
                }
            }).collect::<Vec<ColumnValue>>(), 
            //SUBSCRIBERS
            subscribers : i.subscribers.iter().map(|user| {
                let user = user.clone().unwrap();
                User {
                    email : user.email,  
                    id : user.id,
                    name : user.name
                }
            }).collect::<Vec<User>>(),
        }    
    }).collect::<Vec<Item>>(); 
    return items; 
}