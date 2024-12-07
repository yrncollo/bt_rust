use crossterm::event::{self, KeyEvent};

use crate::app::App;



pub fn handle_key_press(app: &mut App, key_event: KeyEvent){
      
      match key_event.code {
          event::KeyCode::Char('q') => app.exit(),
          _=> {}
          
      }
  }


