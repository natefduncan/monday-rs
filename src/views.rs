use tui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table},
};

use super::objects;

//Menu enum
#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Boards,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Boards => 1,
        }
    }
}

pub fn render_home<'a>() -> Paragraph<'a> {
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

pub fn render_board_list<'a>(
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

pub fn render_board_detail<'a>(
    board : objects::Board
) {
   
}
