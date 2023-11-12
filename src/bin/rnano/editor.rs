use crate::config::EditorConfig;
use crate::cursor::move_cursor;
use crate::Document;
use crate::Position;
use crate::Row;
use crate::Terminal;
use std::time::{Duration, Instant};
use std::{
    io::{self, Error},
    process,
};
use termion::color;
use termion::event::Key;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239);

//errors in editor initialization are propagated upwards.
//errors in editor operation are handled within the run function itself (shutdown).
struct StatusMessage {
    text: String,
    time: Instant,
}
impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

pub struct Editor {
    pub file_name: String,
    pub exited: bool,
    pub offset: Position,
    status_message: StatusMessage,

    //modules in terminal do not obtain this, they are mostly helper functions who DIRECTLY interact with the terminal window through termion.
    pub terminal: Terminal,

    //cursor module functions obtain mutable reference to this.
    pub cursor_position: Position,

    pub current_document: Document,
}

impl Editor {
    pub fn build(config: EditorConfig) -> Result<Editor, io::Error> {
        let mut initial_status = String::from("Press Ctrl-C to quit");
        let current_document = match Document::open(&config.file_name) {
            Ok(doc) => doc,
            Err(_) => {
                initial_status = format!("ERR: Could not open file: {}", &config.file_name);
                Document::default()
            }
        };
        let terminal = Terminal::build()?;
        Ok(Editor {
            file_name: config.file_name,
            exited: false,
            terminal: terminal,
            cursor_position: Position::default(),
            offset: Position::default(),
            current_document,
            status_message: StatusMessage::from(initial_status),
        })
    }

    fn shutdown(&self, _err: Error) {
        Terminal::clear_screen();
        process::exit(1);
    }

    pub fn draw_row(&self, row: &Row) {
        let start: usize = self.offset.x;
        let end = self.terminal.get_dimensions().width as usize + self.offset.x;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_interface(&self) {
        let term_height = self.terminal.get_dimensions().height;
        for terminal_row in 0..term_height {
            Terminal::clear_current_line();
            if let Some(row) = self
                .current_document
                .row(terminal_row as usize + self.offset.y)
            {
                self.draw_row(row);
            } else if self.current_document.is_empty() && terminal_row == term_height / 3 {
                //to prevent overflow
                self.display_welcome();
            } else {
                println!("~\r");
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        if self.exited {
            println!("See ya!\r");
        } else {
            self.draw_interface();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }
        Terminal::show_cursor();
        Terminal::flush()
    }

    fn draw_status_bar(&self) {
        let mut status;
        let width = self.terminal.get_dimensions().width as usize;
        let mut file_name = "[Unnamed]".to_string();
        if let Some(name) = &self.current_document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }
        status = format!("{} - {} lines", file_name, self.current_document.len());

        //in case we scroll to the right, spaces get filled up
        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.current_document.len()
        );
        let len = status.len() + line_indicator.len();
        if width > len {
            status.push_str(&" ".repeat(width - len));
        }
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.get_dimensions().width as usize);
            print!("{}", text);
        }
    }

    fn display_welcome(&self) {
        //ripped off this formatting lol
        let mut welcome_message = String::from("welcome to rnano");
        let term_width = self.terminal.get_dimensions().width;
        let message_length: usize = welcome_message.len();
        let padding = term_width.saturating_sub(message_length as u16) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1) as usize);
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(term_width as usize);
        println!("{}\r", welcome_message);
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.get_dimensions().width as usize;
        let height = self.terminal.get_dimensions().height as usize;
        let offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn handle_keypress(&mut self) -> Result<(), Error> {
        let result = Terminal::read_key()?;
        match result {
            Key::Ctrl('c') => {
                self.exited = true;
            }
            Key::Up
            | Key::Down
            | Key::Right
            | Key::Left
            | Key::PageDown
            | Key::PageUp
            | Key::Home
            | Key::End => {
                move_cursor(
                    &self.terminal,
                    &self.current_document,
                    result,
                    &mut self.cursor_position,
                );
            }
            _ => (),
        };
        self.scroll();
        Ok(())
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
