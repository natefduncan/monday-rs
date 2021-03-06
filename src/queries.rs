use super::monday;
use super::objects::*;
use super::app; 
use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;
use serde_json::{
    map::Map,
    value::Value
}; 


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
        limit: Some(100), 
        newest_first : Some(false),
        page : Some(1)
    };
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

//GROUP LIST
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/group_list.graphql",
    response_derives = "Debug"
)]
struct GroupList;

pub fn group_list(client: &Client, board_id : String) -> Vec<Group> {
    let variables = group_list::Variables { board_id : Some(board_id.parse::<i64>().unwrap()) };
    let res: Response<group_list::ResponseData> =
        monday::query::<GroupList>(&client, variables).expect("Could not execute query.");
    let groups = parse_group_list_response(res);
    return groups;
}

fn parse_group_list_response(res: Response<group_list::ResponseData>) -> Vec<Group> {
    let data = res.data.expect("missing response data.");
    let board = data
        .boards
        .unwrap()
        .into_iter()
        .nth(0)
        .expect("missing first value")
        .unwrap();
    let groups: Vec<Group> = match board.groups {
        Some(arr) => arr
            .iter()
            .map(|group| match group {
                Some(b) => Group {
                    title: b.title.to_owned(),
                    id: b.id.to_owned(),
                },
                None => Group {
                    title: "No Title".to_string(),
                    id: "No ID".to_string(),
                },
            })
            .collect(),
        None => vec![],
    };
    return groups;
}

//ITEM LIST
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/item_list.graphql",
    response_derives = "Debug,Clone"
)]
struct ItemList;

pub fn item_list(client: &Client, board_id: String, group_id: String) -> Vec<Item> {
    let variables = item_list::Variables {
        board_id: Some(board_id.parse::<i64>().expect("can convert to i64")),
        group_id: Some(group_id), 
        limit: Some(100),
        newest_first: Some(false),
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
    let mut items : Vec<Item> = vec![]; 

    //GROUPS
    for group in board.groups.unwrap() {
        let g = group.clone().unwrap();
        for item in g.items.unwrap() {
            let i = item.clone().unwrap(); 
            let mut item_new = Item::new(); 
            item_new.id = i.id.clone();
            item_new.name = i.name.clone();
            //Subscribers
            item_new.subscribers = i
            .subscribers
            .iter()
            .map(|sub| {
                let user = sub.clone().unwrap();
                User {
                    id: user.id,
                    email: "".to_string(),
                    name: "".to_string(),
                }
            })
            .collect::<Vec<User>>();
            items.push(item_new); 
        }
    }
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
type JSON = String;

pub fn item_detail(client: &Client, item_id: String) -> Item {
    let variables = item_detail::Variables {
        item_id: Some(item_id.parse::<i64>().unwrap()),
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
    output.id = item.id;
    output.updated_at = item.updated_at.unwrap();
    //Group
    output.group = Group {
        title: item.group.unwrap().title,
        id : "".to_string()
    };
    //Board
    output.board = Board {
        id : item.board.clone().unwrap().id, 
        name : item.board.clone().unwrap().name
    };
    //Subscribers
    output.subscribers = item
        .subscribers
        .iter()
        .map(|sub| {
            let user = sub.clone().unwrap();
            User {
                id: user.id,
                email: user.email,
                name: user.name,
            }
        })
        .collect::<Vec<User>>();
    //Column Values
    output.column_values = item
        .column_values
        .unwrap()
        .iter()
        .map(|c| {
            let c_val = c.clone().unwrap();
            ColumnValue {
                id: c_val.id,
                text: c_val.text.unwrap_or(String::from("")),
                title: c_val.title,
                type_: c_val.type_,
                additional_info: c_val.additional_info.unwrap_or(String::from("")),
            }
        })
        .collect::<Vec<ColumnValue>>();
    //Updates
    output.updates = item
        .updates
        .unwrap()
        .iter()
        .map(|u| {
            let update = u.clone().unwrap();
            let creator = update.creator.unwrap();
            Update {
                text_body: update.text_body.unwrap_or(String::from("")),
                replies: update
                    .replies
                    .unwrap()
                    .iter()
                    .map(|r| {
                        let reply = r.clone().unwrap();
                        let creator = reply.creator.unwrap();
                        Reply {
                            text_body: reply.text_body.unwrap_or(String::from("")),
                            updated_at: reply.updated_at.unwrap(),
                            creator: User {
                                id: creator.id,
                                email: creator.email,
                                name: creator.name,
                            },
                        }
                    })
                    .collect::<Vec<Reply>>(),
                updated_at: update.updated_at.unwrap(),
                creator: User {
                    id: creator.id,
                    name: creator.name,
                    email: creator.email,
                },
            }
        })
        .collect::<Vec<Update>>();
    return output;
}

//Create Update
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/create_update.graphql",
    response_derives = "Debug,Clone"
)]
struct CreateUpdate;

pub fn create_update(client: &Client, item_id: String, body: String) -> String {
    let variables = create_update::Variables {
        item_id: Some(item_id.parse::<i64>().unwrap()),
        body: body,
    };
    let res: Response<create_update::ResponseData> =
        monday::query::<CreateUpdate>(&client, variables).expect("Could not execute query.");
    let data = res.data.expect("missing response data");
    data.create_update.unwrap().id
}

//Get Column
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/board_columns.graphql",
    response_derives = "Debug,Clone"
)]
struct BoardColumns;

pub fn board_columns(client: &Client, board_id: String) -> Vec<Label> {
    let variables = board_columns::Variables {
        board_id: Some(board_id.parse::<i64>().unwrap()),
    };
    let res: Response<board_columns::ResponseData> =
        monday::query::<BoardColumns>(&client, variables).expect("Could not execute query.");
    parse_board_columns(res)
}

fn parse_board_columns(res: Response<board_columns::ResponseData>) -> Vec<Label> {
    let data = res.data.expect("missing response data");
    let mut labels : Vec<Label> = Vec::new(); 
    let board = data
        .boards
        .unwrap()
        .into_iter()
        .nth(0)
        .expect("missing first value")
        .unwrap();
    for col in board.columns.unwrap().iter() {
        let column = col.clone().unwrap(); 
        let setting : serde_json::Value = serde_json::from_str(&column.settings_str.clone()).unwrap();
        match setting.get("labels") {
            Some(v) => {
                let label_map: Map<String, Value> = v.as_object().unwrap().clone();
                for label in label_map.values() {
                    labels.push(Label {
                        column_id : column.id.clone(),
                        name : label.to_string()
                    }); 
                }
            }, 
            None => {}
        }; 
    }; 
    labels
}

//Change status column
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/change_status.graphql",
    response_derives = "Debug,Clone"
)]
struct ChangeStatus;

pub fn change_status(app : &app::App, value : String) {
    let status_column = app
        .cache
        .boards
        .iter()
        .filter(|board| board.id == app.item_detail.board.id)
        .nth(0)
        .expect("no status column for board");
        
    let variables = change_status::Variables {
        item_id : Some(app.item_detail.id.parse::<i64>().unwrap()), 
        column_id : status_column.status_column_id.clone(), 
        board_id : app.item_detail.board.id.parse::<i64>().unwrap(), 
        value : format!("{{\"label\":\"{}\"}}", value.replace("\"", "")) 
    };
    
    monday::query::<ChangeStatus>(&app.client, variables).expect("Could not execute query.");
}

//Get current user
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/current_user.graphql",
    response_derives = "Debug,Clone"
)]
struct CurrentUser;

pub fn current_user(client : &Client) -> User {
        
    let variables = current_user::Variables {};
    
    let res = monday::query::<CurrentUser>(client, variables).expect("Could not execute query.");
    let data = res.data.expect("no data in response");
    let me = data.me.clone().unwrap(); 
    User {
        id : me.id.clone(), 
        email : me.email.clone(), 
        name : me.name.clone()
    }
}

//CREATE ITEM
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/create_item.graphql",
    response_derives = "Debug,Clone"
)]
struct CreateItem;

pub fn create_item(item_name : String, app : &app::App) -> Item {
        
    let variables = create_item::Variables {
        item_name : Some(item_name), 
        board_id : app.board_detail.id.parse::<i64>().unwrap(), 
        group_id : Some(app.group_detail.id.clone())
    };
    
    let res = monday::query::<CreateItem>(&app.client, variables).expect("Could not execute query.");
    let data = res.data.expect("no data in response");
    let mut item = Item::new();
    item.id = data.create_item.unwrap().id.clone(); 
    item
}

//MOVE ITEM
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/move_item.graphql",
    response_derives = "Debug,Clone"
)]
struct MoveItem;

pub fn move_item(app : &app::App, group_id : String) {
        
    let variables = move_item::Variables {
        item_id : Some(app.item_detail.id.parse::<i64>().unwrap()), 
        group_id : group_id
    };
    
    let res = monday::query::<MoveItem>(&app.client, variables).expect("Could not execute query.");
    let _data = res.data.expect("no data in response");
}


//USER LIST
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/user_list.graphql",
    response_derives = "Debug,Clone"
)]
struct UserList;

pub fn user_list(app : &app::App) -> Vec<User> {
        
    let variables = user_list::Variables {
        board_id : Some(app.board_detail.id.parse::<i64>().unwrap()), 
    };
    
    let res = monday::query::<UserList>(&app.client, variables).expect("Could not execute query.");
    let data = res.data.expect("no data in response");
    let board = data
        .boards
        .unwrap()
        .into_iter()
        .nth(0)
        .expect("missing first value")
        .unwrap();
    board.subscribers.iter().map(|sub| {
        let s = sub.clone().unwrap();
        User {
            id : s.id.clone(),
            name : s.name.clone(), 
            email : s.email.clone(),
        }
    }).collect::<Vec<User>>()
}


//Change user column
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "queries/assign_user.graphql",
    response_derives = "Debug,Clone"
)]
struct AssignUser;

pub fn assign_user(app : &app::App, value : String) {
    let board_cache = app
        .cache
        .boards
        .iter()
        .filter(|board| board.id == app.item_detail.board.id)
        .nth(0)
        .expect("no cache for board");
        
    let variables = assign_user::Variables {
        item_id : Some(app.item_detail.id.parse::<i64>().unwrap()), 
        column_id : board_cache.user_column_id.clone(), 
        board_id : app.board_detail.id.parse::<i64>().unwrap(), 
        value : format!("{{\"personsAndTeams\":[{{\"id\": {}, \"kind\": \"person\"}}]}}", value.replace("\"", "")) 
    };
    
    monday::query::<AssignUser>(&app.client, variables).expect("Could not execute query.");
}