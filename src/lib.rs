pub mod app;
pub mod ui;
mod keybindings;

use std::io;
use ratatui::{prelude::Backend, Terminal};

use app::App;
use ui::render_ui;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>{
    loop {
        terminal.draw(|frame|render_ui(frame, &app))?;
        app.handle_events()?;
        if app.exit{
            break;
        }
    }
    Ok(true)
}
