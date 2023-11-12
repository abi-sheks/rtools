use crate::Position;
use std::io::{self, stdout, Write};
use termion::input::TermRead;
use termion::{color, event::Key};

use termion::raw::{IntoRawMode, RawTerminal};

//attempt at separating the concerns of the editor, this handles all low level terminal setup and management.
//basically encapsulating low level terminal logic and providing an API to editor.rs
pub struct Terminal {
    dimensions: Dimensions,
    _stdout: RawTerminal<io::Stdout>,
}

pub struct Dimensions {
    pub width: u16,
    pub height: u16,
}

impl Terminal {
    pub fn build() -> Result<Terminal, io::Error> {
        let size = termion::terminal_size()?;
        Ok(Terminal {
            dimensions: Dimensions {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn get_dimensions(&self) -> &Dimensions {
        &self.dimensions
    }
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
    pub fn show_cursor() {
        print!("{}", termion::cursor::Show);
    }
    pub fn hide_cursor() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }
    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }
    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }
    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }
}
