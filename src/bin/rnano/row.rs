use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    pub text: String,
    pub len: usize,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        let mut initial_row = Self {
            text: String::from(value),
            len: 0,
        };
        initial_row.update_len();
        initial_row
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

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn update_len(&mut self) {
        self.len = self.text[..].graphemes(true).count();
    }
}
