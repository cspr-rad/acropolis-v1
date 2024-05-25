use ratatui::widgets::ListState;
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub error: Option<String>,
    pub running: bool,
    pub items: Vec<String>,
    pub list_state: ListState,
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            error: None,
            running: true,
            items: vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
            ],
            list_state,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}
