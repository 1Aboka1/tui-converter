use std::io;

use crossterm::event::{Event, KeyCode, read};
use tui::{Terminal, backend::Backend};
use tui_textarea::TextArea;

use crate::{Pages, ui::{draw_init, Layouts}};

pub fn start_event_loop<B>(terminal: &mut Terminal<B>, curr_page: &mut Pages, layouts: Layouts) -> Result<(), io::Error>
where
    B: Backend
{
    let mut textarea = TextArea::default();

    loop {
        let input_widget = textarea.widget();
        if let Event::Key(key) = read()? {
            terminal.draw(|f| {
                f.render_widget(input_widget, layouts.input);
            })?;
            match key.code {
                KeyCode::Esc => break,
                KeyCode::Tab => {
                    curr_page.toggle();
                    draw_init(terminal, curr_page)?;
                }
                _ => { textarea.input(key); },
            };
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
