use crossterm::event::KeyCode;

pub mod app;
pub mod cache;
pub mod components;
pub mod events;
pub mod monday;
pub mod objects;
pub mod queries;
pub mod utils;
pub mod views;

fn main() {
    let app = app::App::new(); 
    let item_id = String::from("1528807035"); 
    let board_id = String::from("679878548"); 
    let column_id = String::from("status"); 
    //app : app::App, item_id : i64, column_id : String, board_id : i64, value : String
    queries::change_status(app, 
        item_id.parse::<i64>().unwrap(), 
        column_id, 
        board_id.parse::<i64>().unwrap(), 
        format!("{{\"label\":\"{}\"}}", "Stuck")
    );
}

fn main2() -> Result<(), Box<dyn std::error::Error>> {
    //Terminal
    let mut terminal = app::start_terminal();
    //Receiver Channel
    let rx = events::start_input_handling();
    //Menu
    let mut app = app::App::new();

    loop {
        //Draw frame
        terminal.draw(|mut rect| match app.active_menu_item {
            views::MenuItem::Home => views::Home::render(&mut rect, &app),
            views::MenuItem::Boards => views::BoardList::render(&mut rect, &app),
            views::MenuItem::Items => views::ItemList::render(&mut rect, &app),
            views::MenuItem::ItemDetail => views::ItemDetail::render(&mut rect, &app),
            views::MenuItem::ItemOptions => views::ItemOptions::render(&mut rect, &mut app),
            views::MenuItem::ItemUpdate => views::ItemUpdate::render(&mut rect, &mut app),
            views::MenuItem::ColumnOptions => views::ColumnOptions::render(&mut rect, &mut app),
            views::MenuItem::StatusOptions => views::StatusOptions::render(&mut rect, &mut app)
        })?;

        //Deal with input
        match rx.recv()? {
            events::Event::Input(event) => {
                //Quit
                if event.code == KeyCode::Esc {
                    app::stop_terminal(&mut terminal);
                    break;
                }

                // Key Input
                events::handle_key_input(event, &mut app);

                //View events
                match app.active_menu_item {
                    views::MenuItem::Home => views::Home.process_input_event(event, &mut app),
                    views::MenuItem::Boards => {
                        views::BoardList.process_input_event(event, &mut app)
                    }
                    views::MenuItem::Items => views::ItemList.process_input_event(event, &mut app),
                    views::MenuItem::ItemDetail => {
                        views::ItemDetail.process_input_event(event, &mut app)
                    }
                    views::MenuItem::ItemOptions => {
                        views::ItemOptions.process_input_event(event, &mut app)
                    }
                    views::MenuItem::ItemUpdate => {
                        views::ItemUpdate.process_input_event(event, &mut app)
                    }, 
                    views::MenuItem::ColumnOptions => {
                        views::ColumnOptions.process_input_event(event, &mut app)
                    }, 
                    views::MenuItem::StatusOptions => {
                        views::StatusOptions.process_input_event(event, &mut app)
                    }
                }
            }
            events::Event::Tick => {}
        }
    }
    Ok(())
}
