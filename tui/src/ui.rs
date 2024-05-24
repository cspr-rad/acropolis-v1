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
                Constraint::Percentage(5),  // Top section height
                Constraint::Percentage(95), // Bottom section height
            ]
            .as_ref(),
        )
        .split(size);

    // Render the welcome text
    let welcome_text =
        Paragraph::new("Welcome to Acropolis! --- Press `Esc`, `Ctrl-C` or `q` to stop running.")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Acropolis")
                    .title_alignment(Alignment::Center),
            )
            .style(Style::default().fg(Color::LightMagenta))
            .alignment(Alignment::Center);
    frame.render_widget(welcome_text, chunks[0]);

    // Define the layout for the bottom section (list and bar chart side by side)
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50), // Left section width (List)
                Constraint::Percentage(50), // Right section width (Bar Chart)
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // Create the list items
    let items: Vec<ListItem> = vec![
        ListItem::new("Item 1"),
        ListItem::new("Item 2"),
        ListItem::new("Item 3"),
    ];

    // Render the list
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("List"));
    frame.render_widget(list, bottom_chunks[0]);

    // Define data for the bar chart
    let data = vec![("A", 10), ("B", 20), ("C", 30), ("D", 40)];

    // Render the bar chart
    let bar_chart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Bar Chart"))
        .data(&data)
        .bar_width(5)
        .bar_gap(2)
        .style(Style::default().fg(Color::Green))
        .value_style(Style::default().fg(Color::Yellow).bg(Color::Black));
    frame.render_widget(bar_chart, bottom_chunks[1]);
}
