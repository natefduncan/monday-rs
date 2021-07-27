use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders, ListItem, List};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Color, Style, Modifier}; 

mod monday; 
mod objects; 
mod queries; 

fn main() -> Result<(), io::Error> {
    let client = monday::get_client().expect("Could not get client.");
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        //Set Chunks
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ].as_ref()
            )
            .split(f.size());
        //Create search block
        let block = Block::default()
             .title("Boards")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        //Create list of boards.
        let boards : Vec<objects::Board> = queries::board_list(&client); 
        let board_items : Vec<ListItem> = boards.iter().map(|x| ListItem::new(x.name.to_owned())).collect(); 
        List::new(board_items)
            .block(Block::default()
                .title("Detail")
                .borders(Borders::ALL)
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");
        })?;
    Ok(())
}


