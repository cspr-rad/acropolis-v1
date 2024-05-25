use ratatui::widgets::ListState;
use std::collections::HashMap;
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub error: Option<String>,
    pub running: bool,
    pub elections: Vec<String>,
    pub list_elections: ListState,
    pub tally: HashMap<String, u64>,
}

impl App {
    pub fn new() -> Self {
        let mut list_elections = ListState::default();
        list_elections.select(Some(0));
        Self {
            error: None,
            running: true,
            elections: vec!["None".to_string()],
            list_elections,
            tally: HashMap::new(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next(&mut self) {
        let i = match self.list_elections.selected() {
            Some(i) => {
                if i >= self.elections.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_elections.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.list_elections.selected() {
            Some(i) => {
                if i == 0 {
                    self.elections.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_elections.select(Some(i));
    }
}
