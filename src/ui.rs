use std::io;
use tui::{
    widgets::{Block, Borders, Tabs},
    layout::{Layout, Constraint, Direction, Rect},
    backend::Backend, text::Spans, style::{Style, Color}, symbols::DOT, Terminal
};
use crate::{tabs::*, Pages};

pub fn draw_init<B>(terminal: &mut Terminal<B>, curr_page: &Pages) -> Result<Layouts, io::Error>
where
    B: Backend,
{
    let mut layouts = Layouts { input: Rect::default() };
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

        use Pages::*;
        match curr_page {
            Conversion => {
                tabs = tabs.select(0);
            },
            Operations => {
                tabs = tabs.select(1);
            },
            Binary => {
                tabs = tabs.select(2);
            },
        }

        f.render_widget(tabs, chunks[0]);

        // Here should be tab match statement(maybe)
       layouts = match curr_page {
            Pages::Conversion => conversion::ui::draw(f, chunks).expect("Couldn't draw conversion page"),
            Pages::Operations => operations::ui::draw(f, chunks).expect("Couldn't draw operations page"),
            Pages::Binary => binary::ui::draw(f, chunks).expect("Couldn't draw binary page"),
        };
    })?;

    Ok(layouts)
}

pub enum _BlockState {
    Focused,
    Unfocused,
}

pub struct Layouts {
    pub input: Rect,
}
