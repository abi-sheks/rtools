use crate::config::EditorConfig;
use crate::Terminal;
use std::{
    io::{self, stdout, Error, Write},
    process,
};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

//errors in editor initialization are propagated upwards.
//errors in editor operation are handled within the run function itself (shutdown).

pub struct Editor {
    pub file_name: String,
    pub exited: bool,
    pub terminal: Terminal,
}

impl Editor {
    pub fn build(config: EditorConfig) -> Result<Editor, io::Error> {
        let terminal = Terminal::build()?;
        Ok(Editor {
            file_name: config.file_name,
            exited: false,
            terminal: terminal,
        })
    }

    fn shutdown(&self, _err: Error) {
        Terminal::clear_screen();
        process::exit(1);
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor();
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        if self.exited {
            println!("See ya!\r");
        } else {
            self.draw_interface();
            print!("{}", termion::cursor::Goto(1, 1));
        }
        Terminal::show_cursor();
        Terminal::flush()
    }

    fn draw_interface(&self) {
        for _ in 0..self.terminal.get_dimensions().height - 1 {
            Terminal::clear_current_line();
            println!("~\r");
        }
    }

    fn handle_keypress(&mut self) -> Result<(), Error> {
        let result = Terminal::read_key()?;
        match result {
            Key::Ctrl('c') => {
                self.exited = true;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn run_editor(&mut self) {

        loop {
            if let Err(e) = self.refresh_screen() {
                self.shutdown(e);
            }
            if self.exited {
                break;
            }
            if let Err(e) = self.handle_keypress() {
                self.shutdown(e);
            }
        }
    }
}
