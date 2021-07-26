use graphql_client; 
use super::monday::MondayStruct; 

struct ActivityLogType {
    account_id : graphql_client::String, 
    created_at : graphql_client::String, 
    data : graphql_client::String, 
    entity : graphql_client::String, 
    event : graphql_client::String, 
    id : graphql_client::String, 
    user_id : graphql_client::String
}; 

enum BoardKind {
    Public, 
    Private, 
    Share
}

struct Column {
    archived : graphql_client::Boolean, 
    id : graphql_client::ID, 
    pos: graphql_client::String,
    settings_str : graphql_client::String, 
    title : graphql_client::String, 
    type : graphql_client::String, 
    width : graphql_client::Int
}; 

struct Group {
    archived : graphql_client::Boolean, 
    color : graphql_client::String, 
    deleted : graphql_client::Boolean, 
    id : graphql_client::ID, 
    items : Vec<Item>, 
    position : graphql_client::String, 
    title : graphql_client::String
}; 

struct Item {
    assets : Vec<Asset>, 
    board : Board, 
    column_values : Vec<ColumnValue>, 
    created_at : Date, 
    creator : User, 
    creator_id : graphql_client::String, 
    group : Group, 
    id : graphql_client::ID, 
    name : graphql_client::String, 
    state : State, 
    subscribers : Vec<User>, 
    updated_at : Date, 
    updates : Vec<Update>
};

struct ColumnValue {
    additional_info : JSON, 
    id : graphql_client::ID, 
    text : graphql_client::String, 
    title : graphql_client::String, 
    type : graphql_client::String, 
    value : JSON
};

struct Date {};
struct JSON {};

struct Asset {
    created_at : Date, 
    file_extension : graphql_client::String, 
    file_size : graphql_client::Int, 
    id : graphql_client::ID, 
    name : graphql_client::String, 
    original_geometry : graphql_client::String, 
    public_url : graphql_client::String, 
    uploaded_by : User,
    url : graphql_client::String, 
    url_thumbnail : graphql_client::String 
}; 

enum FirstDayOfTheWeek { Sunday, Monday}; 

struct Account {
    first_day_of_the_week : FirstDayOfTheWeek, 
    id : graphql_client::Int, 
    logo : graphql_client::String, 
    name : graphql_client::String, 
    plan : Plan, 
    show_timeline_weekends : graphql_client::Boolean, 
    slug : graphql_client::String, 
    tier : graphql_client::String
}; 

struct Plan {
    max_users : graphql_client::Int, 
    period : graphql_client::String, 
    tier : graphql_client::String, 
    version : graphql_client::Int
};

struct User {
    account : Account, 
    birthday : Date, 
    country_code : graphql_client::String, 
    created_at : Date, 
    email : graphql_client::String, 
    enabled : graphql_client::Boolean, 
    id : graphql_client::Int, 
    is_admin : graphql_client::Boolean, 
    is_guest : graphql_client::Boolean, 
    is_pending : graphql_client::Boolean, 
    is_verified : graphql_client::Boolean, 
    is_view_only : graphql_client::Boolean, 
    join_date : Date, 
    location : graphql_client::String, 
    mobile_phone : graphql_client::String, 
    name : graphql_client::String, 
    phone : graphql_client::String, 
    photo_original : graphql_client::String, 
    photo_small : graphql_client::String, 
    photo_thumb : graphql_client::String, 
    photo_thumb_small : graphql_client::String, 
    photo_tiny : graphql_client::String, 
    teams : Vec<Team>, 
    time_zome_identifier : graphql_client::String, 
    title : graphql_client::String, 
    url : graphql_client::String, 
    utc_hours_diff : graphql_client::String
}; 

struct Team {
    id : graphql_client::Int, 
    name : graphql_client::String, 
    picture_url : graphql_client::String, 
    users : Vec<User>
}; 

enum State {All, Active, Archived, Deleted}; 

struct Tag {
    color : graphql_client::String, 
    id : graphql_client::Int, 
    name : graphql_client::String
}; 

struct ISO8601DateTime {}; 

struct Update {
    assets : Vec<Asset>, 
    body : graphql_client::String, 
    created_at : Date, 
    creator : User, 
    creator_id : graphql_client::String, 
    id : graphql_client::ID, 
    item_id : graphql_client::String, 
    replies : Vec<Reply>, 
    text_body : graphql_client::String, 
    updated_at : Date
}; 

struct Reply {
    body : graphql_client::String, 
    created_at : Date, 
    creator : User, 
    creator_id : graphql_client::String, 
    id : graphql_client::ID, 
    text_body : graphql_client::String, 
    updated_at : Date
}; 

struct BoardView {
    id : graphql_client::ID, 
    name : graphql_client::String, 
    settings_str : graphql_client::String, 
    type : graphql_client::String
}; 

struct Workspace {
    description : graphql_client::String, 
    id : graphql_client::Int, 
    kind : WorkspaceKind, 
    name : graphql_client::String
}; 

enum WorkspaceKind { Open, Closed }; 

struct Board {
    activity_logs : Vec<ActivityLogType> 
    board_folder_id : graphql_client::Int, 
    board_kind : BoardKind, 
    columns : Vec<Column>, 
    communication : JSON, 
    description : graphql_client::String, 
    groups : Vec<Group>, 
    id : graphql_client::ID, 
    items : Vec<Item>,
    name : graphql_client::String, 
    owner : User,
    permissions : graphql_client::String, 
    pos : graphql_client::String, 
    state : State, 
    subscribers : Vec<User>, 
    tags : Vec<Tag>
    top_group : Group, 
    updated_at : ISO8601DateTime; 
    updates : Vec<Update>
    views : Vec<BoardView>
    workspace : Workspace, 
    workspace_id : graphql_client::Int
}; 

