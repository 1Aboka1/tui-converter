use std::io;
use tui::{
    widgets::{Block, Borders, Tabs, Paragraph},
    layout::{Layout, Constraint, Direction},
    Frame, backend::Backend, text::{Spans, Span, Text}, style::{Style, Color}, symbols::DOT
};

pub fn draw_init<B>(f: &mut Frame<B>) -> Result<(), io::Error>
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ].as_ref()
        )
        .split(f.size());

    let titles: Vec<_> = ["Conversion", "Operations", "Binary"]
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();
    let mut tabs = Tabs::new(titles)
        .block(Block::default().title("Tabs").borders(Borders::ALL)) 
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Red))
        .divider(DOT);

    tabs = tabs.select(0);
    f.render_widget(tabs, chunks[0]);

    let vert_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(chunks[1]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ]
        )
        .split(vert_chunks[0]);

    let block_left = Block::default()
        .title("Convert")
        .borders(Borders::ALL);
    f.render_widget(block_left, vert_chunks[0]);

    let block_right = Block::default()
        .title("Output")
        .borders(Borders::ALL);
    f.render_widget(block_right, vert_chunks[1]);

    let mut input_string: String = String::from("");
    let lines = Text::from((&input_string).as_str());
    let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Input")).style(Style::default().fg(Color::Green));
    
    f.render_widget(p, left_chunks[0]);
    Ok(())
}
