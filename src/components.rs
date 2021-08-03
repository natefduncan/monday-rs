use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    terminal::{Frame}, 
    backend::{CrosstermBackend}
};
use std::io; 
use super::app; 

pub fn get_default_chunks(rect : &Frame<CrosstermBackend<io::Stdout>>) -> Vec<Rect> {
    let size = rect.size(); 
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(2)
    .constraints(
        [
            Constraint::Length(3),
            Constraint::Min(2),
            Constraint::Length(3),
        ]
        .as_ref(),
    )
    .split(size);
    return chunks; 
}

pub fn get_search_block(app : &app::App) -> Paragraph {
    let search_text: String = app.search.iter().map(|x| x.to_string()).collect::<String>();
    return Paragraph::new(search_text)
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Search")
                .border_type(BorderType::Plain),
        ); 
}

pub fn get_menu_block(app : &app::App) -> Tabs {
    let menu = app.menu_titles
                .iter()
                .map(|t| {
                    let (first, rest) = t.split_at(1);
                    Spans::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect();

    let tabs = Tabs::new(menu)
        .select(app.active_menu_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));
    return tabs; 
}