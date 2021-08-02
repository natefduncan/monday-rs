#[derive(Debug, Clone)]
pub struct ActivityLogType {
    account_id: String,
    created_at: String,
    data: String,
    entity: String,
    event: String,
    id: String,
    user_id: String,
}

#[derive(Debug, Copy, Clone)]
pub enum BoardKind {
    Public,
    Private,
    Share,
}

#[derive(Debug, Clone)]
pub struct Column {
    archived: bool,
    id: String,
    pos: String,
    settings_str: String,
    title: String,
    width: u32,
}

#[derive(Debug, Clone)]
pub struct Group {
    // pub archived: bool,
    // pub color: String,
    // pub deleted: bool,
    // pub id: String,
    // pub items: Vec<Item>,
    // pub position: String,
    pub title: String,
}

impl Group {
    pub fn new() -> Group {
        Group {
            title : "".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    // assets: Vec<Asset>,
    // board: Board,
    pub id : u32,
    pub column_values: Vec<ColumnValue>,
    // pub created_at: Date,
    pub creator: User,
    // creator_id: String,
    pub group: Group,
    pub name: String,
    // state: State,
    pub subscribers: Vec<User>,
    pub updated_at: String,
    pub updates: Vec<Update>,
}

impl Item {
    pub fn new() -> Item {
        Item {
            id : 0, 
            column_values : Vec::new(), 
            creator : User::new(), 
            group : Group::new(),
            name : "".to_string(), 
            subscribers : Vec::new(), 
            updated_at : String::from(""), 
            updates : Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColumnValue {
    // additional_info: JSON,
    pub id: String,
    pub text: String,
    pub title: String,
    pub r#type: String, // value: JSON,
}

#[derive(Debug, Clone)]
pub struct Date {
    pub dt : String
}

impl Date {
    pub fn new() -> Date {
        Date {
            dt : "".to_string()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct JSON {}

#[derive(Debug, Clone)]
pub struct Asset {
    created_at: Date,
    file_extension: String,
    file_size: u32,
    id: String,
    name: String,
    original_geometry: String,
    public_url: String,
    uploaded_by: User,
    url: String,
    url_thumbnail: String,
}

#[derive(Debug, Copy, Clone)]
pub enum FirstDayOfTheWeek {
    Sunday,
    Monday,
}

#[derive(Debug, Clone)]
pub struct Account {
    first_day_of_the_week: FirstDayOfTheWeek,
    id: u32,
    logo: String,
    name: String,
    plan: Plan,
    show_timeline_weekends: bool,
    slug: String,
    tier: String,
}

#[derive(Debug, Clone)]
pub struct Plan {
    max_users: u32,
    period: String,
    tier: String,
    version: u32,
}

#[derive(Debug, Clone)]
pub struct User {
    // account: Account,
    // birthday: Date,
    // country_code: String,
    // created_at: Date,
    pub email: String,
    // enabled: bool,
    pub id: i64,
    // is_admin: bool,
    // is_guest: bool,
    // is_pending: bool,
    // is_verified: bool,
    // is_view_only: bool,
    // join_date: Date,
    // location: String,
    // mobile_phone: String,
    pub name: String,
    // phone: String,
    // photo_original: String,
    // photo_small: String,
    // photo_thumb: String,
    // photo_thumb_small: String,
    // photo_tiny: String,
    // teams: Vec<Team>,
    // time_zome_identifier: String,
    // title: String,
    // url: String,
    // utc_hours_diff: String,
}

impl User {
    pub fn new() -> User {
        User {
            email : "".to_string(), 
            id : 0, 
            name : "".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Team {
    id: u32,
    name: String,
    picture_url: String,
    users: Vec<User>,
}

#[derive(Debug, Copy, Clone)]
pub enum State {
    All,
    Active,
    Archived,
    Deleted,
    None,
}

#[derive(Debug, Clone)]
pub struct Tag {
    color: String,
    id: u32,
    name: String,
}

#[derive(Debug, Copy, Clone)]
pub struct ISO8601DateTime {}

#[derive(Debug, Clone)]
pub struct Update {
    // assets: Vec<Asset>,
    // body: String,
    // created_at: Date,
    creator: User,
    // creator_id: String,
    // id: String,
    // item_id: String,
    replies: Vec<Reply>,
    text_body: String,
    updated_at: String,
}

#[derive(Debug, Clone)]
pub struct Reply {
    body: String,
    created_at: Date,
    creator: User,
    creator_id: String,
    id: String,
    text_body: String,
    updated_at: String,
}

#[derive(Debug, Clone)]
pub struct BoardView {
    id: String,
    name: String,
    settings_str: String,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    description: String,
    id: u32,
    kind: WorkspaceKind,
    name: String,
}

#[derive(Debug, Copy, Clone)]
pub enum WorkspaceKind {
    Open,
    Closed,
}

#[derive(Debug, Clone)]
pub struct Board {
    // activity_logs : Vec<ActivityLogType>
    // board_folder_id : u32,
    // board_kind : BoardKind,
    // columns : Vec<Column>,
    // communication : JSON,
    // description : String,
    // groups : Vec<Group>,
    pub id: String,
    // items : Vec<Item>,
    pub name: String,
    // owner : User,
    // permissions : String,
    // pos : String,
    // state : State,
    // subscribers : Vec<User>,
    // tags : Vec<Tag>
    // top_group : Group,
    // updated_at : ISO8601DateTime;
    // updates : Vec<Update>
    // views : Vec<BoardView>
    // workspace : Workspace,
    // workspace_id : u32
}
