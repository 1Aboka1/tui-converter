use std::io;

use crossterm::event::{Event, KeyCode, read};
use tui::{Terminal, backend::{CrosstermBackend, Backend}};
use tui_textarea::TextArea;

pub fn start_event_loop<B>(terminal: &Terminal<B>) -> Result<(), io::Error>
where
    B: Backend
{
    let mut textarea = TextArea::default();

    loop {
        if let Event::Key(key) = read()? {
            if key.code == KeyCode::Esc {
                break;
            }
        }
    }

    // loop {
    //     let widget = textarea.widget();
    //     f.render_widget(widget, left_chunks[0]);

    //     if let Event::Key(key) = read()? {
    //         if key.code == KeyCode::Esc {
    //             break;
    //         }

    //         textarea.input(key);
    //     }
    // }
    Ok(())
}
