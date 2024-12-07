use std::io;

use crossterm::event::{self, KeyEvent, KeyEventKind};

use crate::keybindings::handle_key_press;

#[derive(Default, Debug)]
pub struct App{

    pub exit: bool
}

impl App {
    
    pub fn new() -> App {

        App { 
            exit: false
        }
        
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        
        match event::read()? {
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {

                handle_key_press(self ,key_event)
            }
            _=>{}
            
        };
        Ok(())
    }

     pub fn exit(&mut self) {
        self.exit = true;
    }
}
