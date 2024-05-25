use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::env;
use std::io;
use std::sync::{Arc, Mutex};
use tui::app::{App, AppResult};
use tui::event::{Event, EventHandler};
use tui::handler::handle_key_events;
use tui::tui::Tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let app = Arc::new(Mutex::new(App::new()));

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    let args: Vec<String> = env::args().collect();
    let app_clone = Arc::clone(&app);
    let _fetch_state = tokio::spawn(async move {
        loop {
            let result = reqwest::get("http://127.0.0.1:8080/fetch_election").await;
            {
                let mut app = app_clone.lock().unwrap();
                match result {
                    Ok(_) => println!("yay!"),
                    Err(e) => {
                        app.error = Some(format!(" Results may not be up to date, {e}"));
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });

    // Start the main loop.
    while app.lock().unwrap().running {
        // Render the user interface.
        tui.draw(&mut app.lock().unwrap())?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.lock().unwrap().tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app.lock().unwrap())?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
