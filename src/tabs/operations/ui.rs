use std::io;
use tui::{
    widgets::{Block, Borders, Tabs, Paragraph, ListItem, List},
    layout::{Layout, Constraint, Direction, Rect},
    Frame, backend::Backend, text::{Spans, Span, Text}, style::{Style, Color, Modifier}, symbols::DOT
};

pub fn draw<B>(f: &mut Frame<B>, main_chunks: Vec<Rect>) -> Result<(), io::Error>
where
    B: Backend,
{
    // Middle chunks: left for input, right for output
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(main_chunks[1]);

    // Conversion chunks
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ]
        )
        .split(middle_chunks[0]);

    let left_vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(45),
                Constraint::Percentage(55),
            ]
        )
        .split(left_chunks[1]);
    
    // Options chunks
    let options_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(45),
                Constraint::Percentage(10),
                Constraint::Percentage(45),
            ].as_ref()
        )
        .split(left_vertical_chunks[0]);

    let block_left = Block::default()
        .title("Convert")
        .borders(Borders::ALL);
    f.render_widget(block_left, middle_chunks[0]);

    let block_right = Block::default()
        .title("Output")
        .borders(Borders::ALL);
    f.render_widget(block_right, middle_chunks[1]);

    // Input widget
    let mut input_string: String = String::from("");
    let lines = Text::from((&input_string).as_str());
    let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Input").border_type(tui::widgets::BorderType::Rounded)).style(Style::default().fg(Color::Green));
    f.render_widget(p, left_chunks[0]);

    // Options for the number being converted
    let left_options = [
        ListItem::new("Binary"), 
        ListItem::new("Decimal").style(Style::default().fg(Color::Blue)), 
        ListItem::new("Octadecimal"),
        ListItem::new("Hexadecimal"),
    ];
    let list = List::new(left_options)
        .block(Block::default().title("From").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    f.render_widget(list, options_chunks[0]);

    // Options for the number to be converted
    let right_options = [
        ListItem::new("Binary").style(Style::default().fg(Color::Blue)), 
        ListItem::new("Decimal"), 
        ListItem::new("Octadecimal"),
        ListItem::new("Hexadecimal"),
    ];
    let list = List::new(right_options)
        .block(Block::default().title("To").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");
    f.render_widget(list, options_chunks[2]);

    // Arrow chunks
    let arrow_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(45),
                Constraint::Percentage(10),
                Constraint::Percentage(45),
            ].as_ref()
        )
        .split(options_chunks[1]);
    let ascii_arrow = Paragraph::new(">>---->")
        .block(Block::default().borders(Borders::empty()));
    f.render_widget(ascii_arrow, arrow_chunks[1]);

    Ok(())
}
