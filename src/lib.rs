pub mod app;
mod keybindings;
mod ui;

use std::io;
use ratatui::{prelude::Backend, Terminal};

use app::App;
use ui::ui;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>{
    loop {
        terminal.draw(|frame|ui(frame, &app))?;
        app.handle_events()?;
        if app.exit{
            break;
        }
    }
    Ok(true)
}
