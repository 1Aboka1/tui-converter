mod ui;
use tui_textarea::TextArea;
use std::{io::{self, Write}, thread, time::Duration};
use tui::{
    backend::CrosstermBackend, 
    Terminal
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, QueueableCommand, cursor
};
use ui::ui::draw_init;

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

    let mut textarea = TextArea::default();

    loop {
        let widget = textarea.widget();
        f.render_widget(widget, left_chunks[0]);

        if let Event::Key(key) = read()? {
            if key.code == KeyCode::Esc {
                break;
            }

            textarea.input(key);
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(()) 
}
