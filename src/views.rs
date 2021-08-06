use super::app;
use super::components;
use super::objects;
use super::queries;
use super::utils;
use super::cache; 

use crossterm::event::{KeyCode, KeyEvent};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, Paragraph, Row, Table, Wrap},
};

//Menu enum
#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Boards,
    Groups, 
    Items,
    ItemDetail,
    ItemOptions,
    ItemUpdate,
    NewItem, 
    ColumnOptions, 
    StatusOptions, 
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Boards => 1,
            MenuItem::Groups => 2,
            MenuItem::Items => 3,
            MenuItem::NewItem => 4, 
            MenuItem::ItemDetail => 4,
            MenuItem::ItemOptions => 4,
            MenuItem::ItemUpdate => 4,
            MenuItem::ColumnOptions => 4, 
            MenuItem::StatusOptions => 4
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Home;

impl Home {
    pub fn render(rect: &mut Frame<CrosstermBackend<io::Stdout>>, app: &app::App) {
        //Default chunks, search, and menu
        let chunks = components::get_default_chunks(&rect);
        let menu_block = components::get_menu_block(&app);

        //Home paragraph
        let home_block = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Welcome")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "Monday-CLI",
                Style::default().fg(Color::LightBlue),
            )]),
            Spans::from(vec![Span::raw("")]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Home")
                .border_type(BorderType::Plain),
        );

        //Render components
        rect.render_widget(menu_block, chunks[0]);
        rect.render_widget(home_block, chunks[1]);
    }

    pub fn keyright(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Boards;
    }

    pub fn keyleft(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::ItemDetail;
    }

    pub fn process_input_event(&self, event: KeyEvent, app: &mut app::App) {
        match event.code {
            KeyCode::Left => self.keyleft(app),
            KeyCode::Right => self.keyright(app),
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BoardList;

impl BoardList {
    pub fn render(rect: &mut Frame<CrosstermBackend<io::Stdout>>, app: &app::App) {
        //Default chunks, search, and menu
        let chunks = components::get_default_chunks(&rect);
        let search_block = components::get_search_block(&app);
        let menu_block = components::get_menu_block(&app);

        //Filter boards
        let filtered = utils::filter_boards(&app.boards, &app.key_input);

        //Board chunks
        let board_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(chunks[1]);

        //Board block
        let board_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Boards")
            .border_type(BorderType::Plain);

        //Create list items
        let list_items: Vec<ListItem> = filtered
            .iter()
            .map(|x| ListItem::new(x.name.to_owned()))
            .collect();

        let selected_board = filtered
            .get(
                app.list_state
                    .selected()
                    .expect("there is always a selected board"),
            )
            .unwrap_or(&objects::Board {
                name: "".to_owned(),
                id: "".to_owned(),
            })
            .clone();

        //Create list component
        let board_list = List::new(list_items).block(board_block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

        //Board Detail Table
        let board_detail = Table::new(vec![Row::new(vec![
            Cell::from(Span::raw(selected_board.id.to_string())),
            Cell::from(Span::raw(selected_board.name)),
        ])])
        .header(Row::new(vec![
            Cell::from(Span::styled(
                "ID",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Name",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Detail")
                .border_type(BorderType::Plain),
        )
        .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

        //Render components
        rect.render_widget(menu_block, chunks[0]);
        rect.render_stateful_widget(board_list, board_chunks[0], &mut app.list_state.clone());
        rect.render_widget(board_detail, board_chunks[1]);
        rect.render_widget(search_block, chunks[2]);
    }

    pub fn keyup(self, app: &mut app::App) {
        if let Some(selected) = app.list_state.selected() {
            let amount_boards = utils::filter_boards(&app.boards, &app.key_input).len();
            if selected > 0 {
                app.list_state.select(Some(selected - 1));
            } else {
                app.list_state.select(Some(amount_boards - 1));
            }
        }
    }

    pub fn keydown(self, app: &mut app::App) {
        if let Some(selected) = app.list_state.selected() {
            let list_length = utils::filter_boards(&app.boards, &app.key_input).len();
            if selected >= list_length - 1 {
                app.list_state.select(Some(0));
            } else {
                app.list_state.select(Some(selected + 1));
            }
        }
    }

    pub fn keyright(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Groups;
    }

    pub fn keyleft(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Home;
    }

    pub fn keyenter(self, app: &mut app::App) {
        let board_filtered = utils::filter_boards(&app.boards, &app.key_input);
        app.active_menu_item = MenuItem::Groups;
        let selected_board = board_filtered
            .get(app.list_state.selected().unwrap())
            .unwrap()
            .clone();
        app.board_detail = selected_board.clone(); 
        app.groups = queries::group_list(&app.client, selected_board.id); 
        app.key_input = Vec::new();
        app.list_state.select(Some(0));
    }

    pub fn process_input_event(&self, event: KeyEvent, app: &mut app::App) {
        match event.code {
            KeyCode::Up => self.keyup(app),
            KeyCode::Down => self.keydown(app),
            KeyCode::Left => self.keyleft(app),
            KeyCode::Right => self.keyright(app),
            KeyCode::Enter => self.keyenter(app),
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GroupList;

impl GroupList {
    pub fn render(rect: &mut Frame<CrosstermBackend<io::Stdout>>, app: &mut app::App) {
        //Default chunks, search, and menu
        let chunks = components::get_default_chunks(&rect);
        let search_block = components::get_search_block(&app);
        let menu_block = components::get_menu_block(&app);

        //Filter items
        let filtered = utils::filter_groups(&app);
        let board_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Group")
            .border_type(BorderType::Plain);

        //Create item list
        let list_items: Vec<ListItem> = filtered
            .iter()
            .map(|x| ListItem::new(x.title.to_owned()))
            .collect();

        let item_list = List::new(list_items).block(board_block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

        //Render components
        rect.render_widget(menu_block, chunks[0]);
        rect.render_stateful_widget(item_list, chunks[1], &mut app.list_state.clone());
        rect.render_widget(search_block, chunks[2]);
    }

    pub fn keyup(self, app: &mut app::App) {
        if let Some(selected) = app.list_state.selected() {
            let list_length = utils::filter_groups(app).len();
            if selected > 0 {
                app.list_state.select(Some(selected - 1));
            } else {
                app.list_state.select(Some(list_length - 1));
            }
        }
    }

    pub fn keydown(self, app: &mut app::App) {
        if let Some(selected) = app.list_state.selected() {
            let list_length = utils::filter_groups(app).len();
            if selected >= list_length - 1 {
                app.list_state.select(Some(0));
            } else {
                app.list_state.select(Some(selected + 1));
            }
        }
    }

    pub fn keyright(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Items;
    }

    pub fn keyleft(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Boards;
    }

    pub fn keyenter(self, app: &mut app::App) {
        let filtered = utils::filter_groups(&app);
        app.active_menu_item = MenuItem::Items;
        let selected_group = filtered
            .get(app.list_state.selected().unwrap())
            .unwrap()
            .clone();
        app.group_detail = selected_group.clone(); 
        app.items = queries::item_list(&app.client, app.board_detail.id.clone(), selected_group.id);
        app.key_input = Vec::new();
        app.list_state.select(Some(0));
    }

    pub fn process_input_event(&self, event: KeyEvent, app: &mut app::App) {
        match event.code {
            KeyCode::Up => self.keyup(app),
            KeyCode::Down => self.keydown(app),
            KeyCode::Left => self.keyleft(app),
            KeyCode::Right => self.keyright(app),
            KeyCode::Enter => self.keyenter(app),
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ItemList;

impl ItemList {
    pub fn render(rect: &mut Frame<CrosstermBackend<io::Stdout>>, app: &mut app::App) {
        //Default chunks, search, and menu
        let chunks = components::get_default_chunks(&rect);
        let search_block = components::get_search_block(&app);
        let menu_block = components::get_menu_block(&app);

        //Filter items
        let filtered = utils::filter_items(app);
        let board_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Item")
            .border_type(BorderType::Plain);

        //Create item list
        let list_items: Vec<ListItem> = filtered
            .iter()
            .map(|x| ListItem::new(x.name.to_owned()))
            .collect();

        let item_list = List::new(list_items).block(board_block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

        //Render components
        rect.render_widget(menu_block, chunks[0]);
        rect.render_stateful_widget(item_list, chunks[1], &mut app.list_state.clone());
        rect.render_widget(search_block, chunks[2]);
    }

    pub fn keyup(self, app: &mut app::App) {
        if let Some(selected) = app.list_state.selected() {
            let amount_boards = utils::filter_items(app).len();
            if selected > 0 {
                app.list_state.select(Some(selected - 1));
            } else {
                app.list_state.select(Some(amount_boards - 1));
            }
        }
    }

    pub fn keydown(self, app: &mut app::App) {
        if let Some(selected) = app.list_state.selected() {
            let list_length = utils::filter_items(app).len();
            if selected >= list_length - 1 {
                app.list_state.select(Some(0));
            } else {
                app.list_state.select(Some(selected + 1));
            }
        }
    }

    pub fn keyright(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::ItemDetail;
    }

    pub fn keyleft(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Groups;
    }

    pub fn keyenter(self, app: &mut app::App) {
        let item_filtered = utils::filter_items(app);
        app.active_menu_item = MenuItem::ItemDetail;
        let selected_item = item_filtered
            .get(app.list_state.selected().unwrap())
            .unwrap()
            .clone();
        app.item_detail = queries::item_detail(&app.client, selected_item.id);
        app.key_input = Vec::new();
        app.list_state.select(Some(0));
    }

    pub fn process_input_event(&self, event: KeyEvent, app: &mut app::App) {
        match event.code {
            KeyCode::Up => self.keyup(app),
            KeyCode::Down => self.keydown(app),
            KeyCode::Left => self.keyleft(app),
            KeyCode::Right => self.keyright(app),
            KeyCode::Enter => self.keyenter(app),
            KeyCode::F(1) => app.f = KeyCode::F(1), //No Filter
            KeyCode::F(2) => app.f = KeyCode::F(2), //Filter By Assigned
            KeyCode::F(3) => { //Create Item in Current Group
                app.active_menu_item = MenuItem::NewItem; 
            }
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ItemDetail;

impl ItemDetail {
    pub fn render(rect: &mut Frame<CrosstermBackend<io::Stdout>>, app: &app::App) {
        //Default chunks, search, and menu
        let chunks = components::get_default_chunks(&rect);
        let menu_block = components::get_menu_block(&app);

        //Board detail block
        let board_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Item")
            .border_type(BorderType::Plain);

        //Span Vec
        let mut column_value_span: Vec<Span> = vec![Span::styled(
            "Column Values: ",
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::LightBlue),
        )];
        for cv in app.item_detail.column_values.iter() {
            if cv.text != "" {
                column_value_span.append(&mut vec![
                    Span::styled(
                        cv.title.clone(),
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .fg(Color::LightCyan),
                    ),
                    Span::raw(": "),
                    Span::styled(
                        cv.text.clone(),
                        Style::default().add_modifier(Modifier::ITALIC),
                    ),
                    Span::raw(" | "),
                ])
            }
        }
        let text = vec![
            Spans::from(vec![
                Span::styled(
                    "Name: ",
                    Style::default()
                        .add_modifier(Modifier::ITALIC)
                        .fg(Color::LightBlue),
                ),
                Span::raw(app.item_detail.name.clone()),
            ]),
            Spans::from(vec![
                Span::styled(
                    "Subscribers: ",
                    Style::default()
                        .add_modifier(Modifier::ITALIC)
                        .fg(Color::LightBlue),
                ),
                Span::raw(
                    app.item_detail
                        .subscribers
                        .iter()
                        .map(|sub| sub.name.clone())
                        .collect::<Vec<String>>()
                        .join(", "),
                ),
            ]),
            Spans::from(vec![
                Span::styled(
                    "Updated at: ",
                    Style::default()
                        .add_modifier(Modifier::ITALIC)
                        .fg(Color::LightBlue),
                ),
                Span::raw(app.item_detail.updated_at.clone()),
            ]),
            Spans::from(vec![
                Span::styled(
                    "Group: ",
                    Style::default()
                        .add_modifier(Modifier::ITALIC)
                        .fg(Color::LightBlue),
                ),
                Span::raw(app.item_detail.group.title.clone()),
            ]),
            Spans::from(vec![
                Span::styled(
                    "Updates: ",
                    Style::default()
                        .add_modifier(Modifier::ITALIC)
                        .fg(Color::LightBlue),
                ),
                Span::raw(
                    app.item_detail
                        .updates
                        .iter()
                        .map(|update| update.text_body.clone())
                        .collect::<Vec<String>>()
                        .join(" | "),
                ),
            ]),
            Spans::from(column_value_span),
        ];

        //Paragraph
        let p = Paragraph::new(text)
            .block(board_block)
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        //Render Components
        rect.render_widget(menu_block, chunks[0]);
        rect.render_widget(p, chunks[1]);
    }

    pub fn keyright(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Home;
    }

    pub fn keyleft(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Items;
    }

    pub fn keyenter(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::ItemOptions;
        app.key_input = Vec::new();
        app.list_state.select(Some(0));
    }

    pub fn process_input_event(&self, event: KeyEvent, app: &mut app::App) {
        match event.code {
            KeyCode::Left => self.keyleft(app),
            KeyCode::Right => self.keyright(app),
            KeyCode::Enter => self.keyenter(app),
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ItemOptions;

impl ItemOptions {
    pub fn render(rect: &mut Frame<CrosstermBackend<io::Stdout>>, app: &mut app::App) {
        //Default chunks, search, and menu
        let chunks = components::get_default_chunks(&rect);

        let items = [ListItem::new("Add Update"), ListItem::new("Change Status")];

        let option_list = List::new(items)
            .block(Block::default().title("Options").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        rect.render_stateful_widget(option_list, chunks[1], &mut app.list_state);
    }

    pub fn keyright(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Home;
    }

    pub fn keyleft(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Items;
    }

    pub fn keyup(self, app: &mut app::App) {
        match app.list_state.selected().unwrap() {
            0 => app.list_state.select(Some(1)),
            1 => app.list_state.select(Some(0)),
            _ => app.list_state.select(Some(0)),
        }
    }

    pub fn keydown(self, app: &mut app::App) {
        match app.list_state.selected().unwrap() {
            0 => app.list_state.select(Some(1)),
            1 => app.list_state.select(Some(0)),
            _ => app.list_state.select(Some(0)),
        }
    }

    pub fn process_input_event(&self, event: KeyEvent, app: &mut app::App) {
        match event.code {
            KeyCode::Left => self.keyleft(app),
            KeyCode::Right => self.keyright(app),
            KeyCode::Up => self.keyup(app),
            KeyCode::Down => self.keydown(app),
            KeyCode::Enter => match app.list_state.selected().unwrap() {
                0 => app.active_menu_item = MenuItem::ItemUpdate,
                1 => {
                    if app.cache.board_has_meta(app.item_detail.board.id.clone()) {
                        app.status_labels = queries::board_columns(&app.client, app.item_detail.board.id.clone()); 
                        app.active_menu_item = MenuItem::StatusOptions; 
                    } else {
                        app.active_menu_item = MenuItem::ColumnOptions; 
                    }
                }, 
                _ => {}
            },
            KeyCode::Char('U') => {
                app.active_menu_item = MenuItem::ItemUpdate;
                app.key_input = Vec::new();
            }
            KeyCode::Char('S') => {}
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ItemUpdate;

impl ItemUpdate {
    pub fn render(rect: &mut Frame<CrosstermBackend<io::Stdout>>, app: &mut app::App) {
        //Default chunks, search, and menu
        let chunks = components::get_default_chunks(&rect);

        //Key input as string
        let update_text: String = app
            .key_input
            .iter()
            .map(|x| x.to_string())
            .collect::<String>();

        //Span Vec
        let update_span = vec![Spans::from(vec![
            Span::styled(
                "Update: ",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::LightBlue),
            ),
            Span::raw(update_text),
        ])];

        let p = Paragraph::new(update_span)
            .style(Style::default())
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Add Update")
                    .border_type(BorderType::Plain),
            )
            .wrap(Wrap { trim: true });

        rect.render_widget(p, chunks[1]);
    }

    pub fn keyright(self, app: &mut app::App) {
        app.key_input = Vec::new();
        app.active_menu_item = MenuItem::Home;
    }

    pub fn keyleft(self, app: &mut app::App) {
        app.key_input = Vec::new();
        app.active_menu_item = MenuItem::ItemOptions;
    }

    pub fn process_input_event(&self, event: KeyEvent, app: &mut app::App) {
        match event.code {
            KeyCode::Left => self.keyleft(app),
            KeyCode::Right => self.keyright(app),
            KeyCode::Enter => {
                //Key input as string
                let update_text: String = app
                    .key_input
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<String>();
                // GraphQL create update
                queries::create_update(&app.client, app.item_detail.id.clone(), update_text);
                // Get Item Detail again
                app.item_detail = queries::item_detail(&app.client, app.item_detail.id.clone());
                //Change menu back to Item Detail
                app.active_menu_item = MenuItem::ItemDetail;
            }
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ColumnOptions;

impl ColumnOptions {
    pub fn render(rect: &mut Frame<CrosstermBackend<io::Stdout>>, app: &mut app::App) {
        //Default chunks, search, and menu
        let chunks = components::get_default_chunks(&rect);

        // Find status column
        let status_columns : Vec<objects::ColumnValue> = app.item_detail.column_values.iter().filter(|cv| cv.type_ == String::from("color")).cloned().collect::<Vec<objects::ColumnValue>>();
        let items = status_columns.iter().map(|x| ListItem::new(x.title.clone())).collect::<Vec<ListItem>>(); 

        let option_list = List::new(items)
            .block(Block::default().title("Select Status Column").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        rect.render_stateful_widget(option_list, chunks[1], &mut app.list_state);
    }

    pub fn keyright(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Home;
    }

    pub fn keyleft(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Items;
    }

    pub fn keyup(self, app: &mut app::App) {
        let status_columns : Vec<objects::ColumnValue> = app.item_detail.column_values.iter().filter(|cv| cv.type_ == String::from("color")).cloned().collect::<Vec<objects::ColumnValue>>();
        if let Some(selected) = app.list_state.selected() {
            if selected == 0 {
                app.list_state.select(Some(status_columns.len() - 1));
            } else {
                app.list_state.select(Some(selected - 1));
            }
        }
    }

    pub fn keydown(self, app: &mut app::App) {
        let status_columns : Vec<objects::ColumnValue> = app.item_detail.column_values.iter().filter(|cv| cv.type_ == String::from("color")).cloned().collect::<Vec<objects::ColumnValue>>();
        if let Some(selected) = app.list_state.selected() {
            if selected >= status_columns.len() - 1 {
                app.list_state.select(Some(0));
            } else {
                app.list_state.select(Some(selected + 1));
            }
        }
    }

    pub fn process_input_event(&self, event: KeyEvent, app: &mut app::App) {
        match event.code {
            KeyCode::Left => self.keyleft(app),
            KeyCode::Right => self.keyright(app),
            KeyCode::Up => self.keyup(app),
            KeyCode::Down => self.keydown(app),
            KeyCode::Enter => {
                let status_columns : Vec<objects::ColumnValue> = app.item_detail.column_values.iter().filter(|cv| cv.type_ == String::from("color")).cloned().collect::<Vec<objects::ColumnValue>>();
                let column = status_columns.get(
                    app.list_state.selected().unwrap()
                ).unwrap();
                app.cache.update_board_meta(cache::BoardMeta {
                    id : app.item_detail.board.id.clone(), 
                    status_column_id : column.id.clone()
                }); 
                cache::write(&app.cache).expect("could not write cache");
                app.status_labels = queries::board_columns(&app.client, app.item_detail.board.id.clone()); 
                app.active_menu_item = MenuItem::StatusOptions; 
            }, 
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StatusOptions;

impl StatusOptions {
    pub fn render(rect: &mut Frame<CrosstermBackend<io::Stdout>>, app: &mut app::App) {
        let chunks = components::get_default_chunks(&rect); 
        let items = app.status_labels.iter().map(|x| ListItem::new(x.name.clone().replace("\"", ""))).collect::<Vec<ListItem>>(); 

        let option_list = List::new(items)
            .block(Block::default().title("Change Status").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        rect.render_stateful_widget(option_list, chunks[1], &mut app.list_state);
    }

    pub fn keyright(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::Home;
    }

    pub fn keyleft(self, app: &mut app::App) {
        app.active_menu_item = MenuItem::ItemOptions;
    }

    pub fn keyup(self, app: &mut app::App) {
        if let Some(selected) = app.list_state.selected() {
            if selected == 0 {
                app.list_state.select(Some(app.status_labels.len() - 1));
            } else {
                app.list_state.select(Some(selected - 1));
            }
        }
    }

    pub fn keydown(self, app: &mut app::App) {
        if let Some(selected) = app.list_state.selected() {
            if selected >= app.status_labels.len() - 1 {
                app.list_state.select(Some(0));
            } else {
                app.list_state.select(Some(selected + 1));
            }
        }
    }

    pub fn process_input_event(&self, event: KeyEvent, app: &mut app::App) {
        match event.code {
            KeyCode::Left => self.keyleft(app),
            KeyCode::Right => self.keyright(app),
            KeyCode::Up => self.keyup(app),
            KeyCode::Down => self.keydown(app),
            KeyCode::Enter => {
                let selected_label = app.status_labels.get(
                    app.list_state.selected().unwrap()
                ).unwrap(); 
                queries::change_status(app, selected_label.name.clone()); 
                app.item_detail = queries::item_detail(&app.client, app.item_detail.id.clone()); 
                app.active_menu_item = MenuItem::ItemDetail; 
            },
            _ => {}
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NewItem;

impl NewItem {
    pub fn render(rect: &mut Frame<CrosstermBackend<io::Stdout>>, app: &mut app::App) {
        //Default chunks, search, and menu
        let chunks = components::get_default_chunks(&rect);

        //Key input as string
        let update_text: String = app
            .key_input
            .iter()
            .map(|x| x.to_string())
            .collect::<String>();

        //Span Vec
        let update_span = vec![Spans::from(vec![
            Span::styled(
                "Item Name: ",
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::LightBlue),
            ),
            Span::raw(update_text),
        ])];

        let p = Paragraph::new(update_span)
            .style(Style::default())
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Create Item")
                    .border_type(BorderType::Plain),
            )
            .wrap(Wrap { trim: true });

        rect.render_widget(p, chunks[1]);
    }

    pub fn keyright(self, app: &mut app::App) {
        app.key_input = Vec::new();
        app.active_menu_item = MenuItem::ItemDetail;
    }

    pub fn keyleft(self, app: &mut app::App) {
        app.key_input = Vec::new();
        app.active_menu_item = MenuItem::Items;
    }

    pub fn process_input_event(&self, event: KeyEvent, app: &mut app::App) {
        match event.code {
            KeyCode::Left => self.keyleft(app),
            KeyCode::Right => self.keyright(app),
            KeyCode::Enter => {
                //Key input as string
                let item_text: String = app
                    .key_input
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<String>();
                // GraphQL create item
                let new_item = queries::create_item(item_text, &app);
                // Get Item Detail again
                app.item_detail = queries::item_detail(&app.client, new_item.id.clone());
                //Change menu back to Item Detail
                app.active_menu_item = MenuItem::ItemDetail;
            }
            _ => {}
        }
    }
}
