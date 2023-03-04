use std::io;
use tui::{
    widgets::{Block, Borders, Tabs, Paragraph, ListItem, List},
    layout::{Layout, Constraint, Direction},
    Frame, backend::Backend, text::{Spans, Span, Text}, style::{Style, Color, Modifier}, symbols::DOT, Terminal
};
use crate::tabs;

pub enum Pages {
    Conversion,
    Operations,
    Binary,
}

pub fn draw_init<B>(terminal: &mut Terminal<B>) -> Result<(), io::Error>
where
    B: Backend,
{
    terminal.draw(|f| {
        // Main chunks
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

        // Tabs
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

        // Here should be tab match statement(maybe)
        tabs::conversion::ui::draw(f, chunks).expect("Couldn't draw conversion page");
    })?;

    Ok(())
}
