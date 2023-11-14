use unicode_segmentation::UnicodeSegmentation;

use crate::editor::SearchDirection;
use crate::highlighting;

#[derive(Default)]
pub struct Row {
    pub text: String,
    highlighting : Vec<highlighting::Type>,
    pub len: usize,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self {
            text: String::from(value),
            highlighting : Vec::new(),
            len: value.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = std::cmp::min(end, self.text.len());
        let start = std::cmp::min(start, end);
        let mut result = String::new();
        for grapheme in self.text[..].graphemes(true).skip(start).take(end - start) {
            if grapheme == "\t" {
                result.push_str(" ");
            } else {
                result.push_str(grapheme)
            }
        }
        result
    }

    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        //basically, string.find() returns byte index, but we want grapheme index (complex languages, grapheme != byte). so we iterate over every grapheme, using a combination of grapheme_indices (returns (g_index, g) and enumerate on g, (byte_index, g) to compare byte_index but return grapheme index.)
        if at > self.len {
            return None;
        }
        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.len
        } else {
            at
        };
        let substring: String = self.text[..].graphemes(true).skip(start).take(end-start).collect();
        let matching_byte_index = if direction == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };
        if let Some(mbi) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in substring[..].grapheme_indices(true).enumerate() {
                if mbi == byte_index {
                    return Some(start + grapheme_index);
                }
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn insert(&mut self, c: char, position: usize) {
        if position >= self.len() {
            self.text.push(c);
            self.len += 1;
            return;
        }
        let mut result = String::new();
        let mut length = 0;
        for (index, grapheme) in self.text[..].graphemes(true).enumerate() {
            length += 1;
            if index == position {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.text = result;
    }
    pub fn delete(&mut self, position: usize) {
        if position >= self.len() {
            return;
        }
        let mut result = String::new();
        let mut length = 0;
        for (index, grapheme) in self.text[..].graphemes(true).enumerate() {
            if index != position {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.text = result;
    }
    pub fn append(&mut self, new: &Self) {
        self.text = format!("{}{}", self.text, new.text);
        self.len += new.len;
    }

    pub fn highlight(&mut self) {
        let mut highlighting = Vec::new();
        for c in self.text.chars() {
            if c.is_ascii_digit() {
                highlighting.push(highlighting::Type::Number);
            } else {
                highlighting.push(highlighting::Type::None)
            }
        }
        self.highlighting = highlighting;
    }

    pub fn split_row(&mut self, position: usize) -> Self {
        let mut row: String = String::new();
        let mut length = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length = 0;
        for (index, grapheme) in self.text[..].graphemes(true).enumerate() {
            if index < position {
                length += 1;
                row.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }

        self.text = row;
        self.len = length;
        Self {
            text: splitted_row,
            len: splitted_length,
            highlighting : Vec::new(),
        }
    }
}
