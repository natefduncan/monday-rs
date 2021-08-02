use tui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table},
};

use super::app;
use super::objects;
use super::queries;
use super::utils;

//Menu enum
#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Boards,
    Items,
    ItemDetail
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Boards => 1,
            MenuItem::Items => 2,
            MenuItem::ItemDetail => 3
        }
    }
}

pub struct Home {}

impl Home {
    pub fn render<'a>() -> Paragraph<'a> {
        let home = Paragraph::new(vec![
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
        home
    }
}

pub struct BoardList;

impl BoardList {
    pub fn render<'a>(
        board_vec: &Vec<objects::Board>,
        board_list_state: &ListState,
    ) -> (List<'a>, Table<'a>) {
        let board_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Boards")
            .border_type(BorderType::Plain);

        let list_items: Vec<ListItem> = board_vec
            .iter()
            .map(|x| ListItem::new(x.name.to_owned()))
            .collect();
        let selected_board = board_vec
            .get(
                board_list_state
                    .selected()
                    .expect("there is always a selected board"),
            )
            .unwrap_or(&objects::Board {
                name: "".to_owned(),
                id: "".to_owned(),
            })
            .clone();

        let board_list = List::new(list_items).block(board_block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

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

        (board_list, board_detail)
    }

    pub fn keyup(app: &mut app::App) {
        if let Some(selected) = app.list_state.selected() {
            let amount_boards = utils::filter_boards(&app.boards, &app.search).len();
            if selected > 0 {
                app.list_state.select(Some(selected - 1));
            } else {
                app.list_state.select(Some(amount_boards - 1));
            }
        }
    }

    pub fn keydown(app: &mut app::App) {
        if let Some(selected) = app.list_state.selected() {
            let list_length = utils::filter_boards(&app.boards, &app.search).len();
            if selected >= list_length - 1 {
                app.list_state.select(Some(0));
            } else {
                app.list_state.select(Some(selected + 1));
            }
        }
    }

    pub fn keyenter(app: &mut app::App) {
        let board_filtered = utils::filter_boards(&app.boards, &app.search);
        app.active_menu_item = MenuItem::Items;
        let selected_board = board_filtered
            .get(app.list_state.selected().unwrap())
            .unwrap()
            .clone();
        app.items = queries::item_list(&app.client, selected_board.id);
        app.search = Vec::new();
    }
}

pub struct ItemList;

impl ItemList {
    pub fn render<'a>(items: &Vec<objects::Item>, list_state: &ListState) -> List<'a> {
        let board_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Items")
            .border_type(BorderType::Plain);

        let list_items: Vec<ListItem> = items
            .iter()
            .map(|x| ListItem::new(x.name.to_owned()))
            .collect();
        let selected_item = items
            .get(
                list_state
                    .selected()
                    .expect("there is always a selected group"),
            )
            .unwrap_or(&objects::Item::new())
            .clone();

        let item_list = List::new(list_items).block(board_block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

        item_list
    }

    pub fn keyup(app: &mut app::App) {
        if let Some(selected) = app.list_state.selected() {
            let amount_boards = utils::filter_items(&app.items, &app.search).len();
            if selected > 0 {
                app.list_state.select(Some(selected - 1));
            } else {
                app.list_state.select(Some(amount_boards - 1));
            }
        }
    }

    pub fn keydown(app: &mut app::App) {
        if let Some(selected) = app.list_state.selected() {
            let list_length = utils::filter_items(&app.items, &app.search).len();
            if selected >= list_length - 1 {
                app.list_state.select(Some(0));
            } else {
                app.list_state.select(Some(selected + 1));
            }
        }
    }

    pub fn keyenter(app : &mut app::App) {
        let item_filtered = utils::filter_items(&app.items, &app.search);
        app.active_menu_item = MenuItem::ItemDetail;
        let selected_item = item_filtered
            .get(app.list_state.selected().unwrap())
            .unwrap()
            .clone();
        println!("{:?}", app.list_state.selected()); 
        println!("{:?}", selected_item); 
        app.item_detail = queries::item_detail(&app.client, selected_item.id);
        app.search = Vec::new();        

    }
}
