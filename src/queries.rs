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
    let variables = board_list::Variables { limit: Some(50) };
    let res: Response<board_list::ResponseData> =
        monday::query::<BoardList>(&client, variables).expect("Could not execute query.");
    let boards = parse_board_list_response(res);
    return boards;
}

fn parse_board_list_response(res: Response<board_list::ResponseData>) -> Vec<Board> {
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

//ITEM LIST
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/item_list.graphql",
    response_derives = "Debug,Clone"
)]
struct ItemList;

pub fn item_list(client: &Client, board_id: String) -> Vec<Item> {
    let variables = item_list::Variables {
        board_id: Some(board_id.parse::<i64>().expect("can convert to i64")),
        limit: Some(50),
        newest_first: Some(true),
        page: Some(1),
    };
    let res: Response<item_list::ResponseData> =
        monday::query::<ItemList>(&client, variables).expect("Could not execute query.");
    parse_board_detail_response(res)
}

fn parse_board_detail_response(res: Response<item_list::ResponseData>) -> Vec<Item> {
    let data = res.data.expect("missing response data.");
    let board = data
        .boards
        .unwrap()
        .into_iter()
        .nth(0)
        .expect("missing first value")
        .unwrap();
    //ITEMS
    let items = board
        .items
        .unwrap()
        .iter()
        .map(|item| {
            let i = item.clone().unwrap();
            let mut item = Item::new();
            item.id = i.id.parse::<u32>().unwrap().clone(); 
            item.name = i.name.clone(); 
            item
        })
        .collect::<Vec<Item>>();
    return items;
}

//ITEM DETAIL
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/item_detail.graphql",
    response_derives = "Debug,Clone"
)]
struct ItemDetail;

type Date = String; 

pub fn item_detail(client: &Client, item_id: u32) -> Item {
    let variables = item_detail::Variables {
        item_id: Some(item_id as i64),
    };
    let res: Response<item_detail::ResponseData> =
        monday::query::<ItemDetail>(&client, variables).expect("Could not execute query.");
    parse_item_detail_response(res)
}

fn parse_item_detail_response(res: Response<item_detail::ResponseData>) -> Item {
    let data = res.data.expect("missing response data.");
    let item = data
        .items
        .unwrap()
        .into_iter()
        .nth(0)
        .expect("missing first value")
        .unwrap();
    let mut output = Item::new();
    output.name = item.name;
    output.id = item.id.parse::<u32>().unwrap(); 
    output.updated_at = item.updated_at.unwrap(); 
    return output; 

}
