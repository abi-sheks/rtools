use crate::config::SortConfig;
use std::io;
use std::fs;
use std::time::{Instant, Duration};

#[derive(Default)]
pub struct Sort {
    pub results : Vec<String>,
}

impl Sort {
    
    pub fn sort_file(&mut self, config : SortConfig) -> Result<Duration, io::Error> {
        let start = Instant::now();
        let file_results = fs::read_to_string(&config.file_name)?;
        let mut lines : Vec::<&str> = file_results.lines().collect();
        //need to convert array of slices to array of strings.
        lines.sort();
        let mut lines_as_string : Vec<String> = Vec::new();
        for line in lines {
            lines_as_string.push(line.to_string());
        }
        self.results = lines_as_string;
        Ok(start.elapsed())
    }

    pub fn print_results(&self) {
        for result in self.results.iter() {
            println!("{}", result)
        }
    }
}