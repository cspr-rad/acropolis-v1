use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{BarChart, Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let size = frame.size();

    // Define the layout for the top and bottom sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(15),  // Top section height
                Constraint::Percentage(85), // Bottom section height
            ]
            .as_ref(),
        )
        .split(size);

    // Render the welcome text
    let text = match &app.error {
        Some(e) => {
            format!("Welcome to Acropolis! --- Press `Esc`, `Ctrl-C` or `q` to stop running.\n{e}")
        }
        None => {
            "Welcome to Acropolis! --- Press `Esc`, `Ctrl-C` or `q` to stop running.".to_string()
        }
    };
    let text = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Acropolis")
                .title_alignment(Alignment::Center),
        )
        .style(Style::default().fg(Color::LightMagenta))
        .alignment(Alignment::Center);
    frame.render_widget(text, chunks[0]);

    // Define the layout for the bottom section (list and bar chart side by side)
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50), // Left section width (List)
                // Constraint::Percentage(25),
                Constraint::Percentage(50), // Right section width (Bar Chart)
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // Create the list items
    let elections: Vec<ListItem> = app
        .elections
        .iter()
        .map(|i| ListItem::new(i.as_str()))
        .collect();

    // Render the list
    let elections_window = List::new(elections.clone())
        .block(Block::default().borders(Borders::ALL).title("Elections"))
        .highlight_style(Style::default().bg(Color::LightBlue));
    frame.render_stateful_widget(elections_window, bottom_chunks[0], &mut app.list_elections);

    // Render the list
    // let votes = List::new(items)
    //     .block(Block::default().borders(Borders::ALL).title("Legit Votes"))
    //     .highlight_style(Style::default().bg(Color::LightBlue));
    // frame.render_stateful_widget(votes, bottom_chunks[1], &mut app.list_state);

    // Define data for the bar chart
    let tally: Vec<(&str, u64)> = app.tally.iter().map(|(k, v)| (k.as_str(), *v)).collect();

    // Render the bar chart
    let bar_chart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Bar Chart"))
        .data(&tally)
        .bar_width(5)
        .bar_gap(2)
        .style(Style::default().fg(Color::Green))
        .value_style(Style::default().fg(Color::Yellow).bg(Color::Black));
    frame.render_widget(bar_chart, bottom_chunks[1]);
}
