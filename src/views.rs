use tui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Paragraph, List, ListState, Table, ListItem, Block, Borders, BorderType, Row, Cell}
};

use super::objects;

//Menu enum
#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Boards,
    Detail
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Boards => 1,
            MenuItem::Detail => 2, 
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
        board_list_state : &ListState,
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

    pub fn keyup() {

    }

    pub fn keydown() {

    }

    pub fn keyenter() {
        
    }
}

pub struct BoardDetail; 

impl BoardDetail {
    pub fn render<'a>(
        board_detail: &Vec<objects::Group>, 
        group_list_state : &ListState, 
    ) -> (List<'a>, Table<'a>) {
        let board_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Groups")
            .border_type(BorderType::Plain);
    
        let list_items: Vec<ListItem> = board_detail
            .iter()
            .map(|x| ListItem::new(x.title.to_owned()))
            .collect();
        let selected_group = board_detail
            .get(
                group_list_state
                    .selected()
                    .expect("there is always a selected group"),
            )
            .unwrap_or(&objects::Group {
                title: "".to_owned(),
                id: "".to_owned(),
                archived : false, 
                color : "".to_owned(), 
                position : "".to_owned(), 
                deleted : false, 
                items : vec![]
            })
            .clone();
    
        let group_list = List::new(list_items).block(board_block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );
    
        let group_items = Table::new(selected_group.items.iter().map(|x| {
            Row::new(vec![
                Cell::from(Span::raw(x.name.to_owned()))
            ])
        }).collect::<Vec<Row>>()) 
        .header(Row::new(vec![
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
    
        (group_list, group_items)
    }

    pub fn keyup() {

    }

    pub fn keydown() {

    }

    pub fn keyenter() {
        
    }
}

