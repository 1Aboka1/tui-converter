mod ui;
mod event;
mod tabs;
use std::io;
use event::start_event_loop;
use tui::{
    backend::CrosstermBackend, 
    Terminal
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ui::draw_init;

fn main() -> Result<(), io::Error> {
    let mut curr_page = Pages::Conversion;
    start_ui(&mut curr_page)?;

    Ok(())
}

fn start_ui(curr_page: &mut Pages) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let layouts = draw_init(&mut terminal, &curr_page)?;

    start_event_loop(&mut terminal, curr_page, layouts)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(()) 
}

pub enum Pages {
    Conversion,
    Operations,
    Binary,
}

impl Pages {
    pub fn toggle(&mut self) {
        use Pages::*;
        match self {
            Conversion => {
                *self = Operations;
            }, 
            Operations => {
                *self = Binary;
            },
            Binary => {
                *self = Conversion;
            },
        };
    }
}
