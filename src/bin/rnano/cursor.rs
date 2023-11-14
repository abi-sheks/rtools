use termion::event::Key;

use crate::{Document, Terminal};

#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub fn move_cursor(terminal : &Terminal, document: &Document, key: Key, cursor_position: &mut Position) {
    let Position { mut y, mut x } = cursor_position;
    let terminal_height = terminal.get_dimensions().height as usize;
    let height = document.len();
    let mut width = if let Some(row) = document.row(y) {
        row.len()
    } else {
        0
    };
    match key {
        Key::Up => y = y.saturating_sub(1),
        Key::Down => {
            if y < height {
                y = y.saturating_add(1);
            }
        }
        Key::Left => {
            if x > 0 {
                x -= 1;
            } else if y > 0 {
                y -= 1;
                if let Some(row) = document.row(y) {
                    x = row.len();
                } else {
                    x = 0;
                }
            }
        } 
        Key::Right => {
            if x < width {
                x += 1;
            } else if y < height {
                y += 1;
                x = 0;
            }
        }
        Key::PageUp => {
            y = if y > terminal_height {
                y.saturating_sub(terminal_height)
            } else {
                0
            }
        },
        Key::PageDown =>{
            y = if y.saturating_add(terminal_height) < height {
                y.saturating_add(terminal_height)
            } else {
                height
            }
        },
        Key::Home => x = 0,
        Key::End => x = width,
        _ => (),
    };
    width = if let Some(row) = document.row(y) {
        row.len()
    } else {
        0
    };
    if x > width {
        x = width;
    }
    *cursor_position = Position { x, y };
}
