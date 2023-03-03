mod ui;
mod event;
mod tabs;
use std::{io, thread, time::Duration};
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
    start_ui()?;

    Ok(())
}

fn start_ui() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    terminal.draw(|f| {
        draw_init(f).expect("Couldn't draw");
    })?;

    thread::sleep(Duration::from_millis(3000));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(()) 
}
