use std::{
    fs,
    path::{Path, PathBuf}, io,
    time::{Instant, Duration},
};

use colored::Colorize;

pub struct Search {
    //query stored as it is used in print_results
    query : String,
    results: Vec<SearchResult>,
}

struct SearchResult {
    content : String,
    file_name : Box<PathBuf>,
}

impl Search {

    pub fn new() -> Search {
        Search {results : Vec::new(), query : String::from("")}
    }
    pub fn print_results(&self) -> () {
        for result in self.results.iter() {
            let start_index = result.content.find(&self.query[..]).unwrap();
            let end_index = start_index + self.query.len();
            println!("In file : {:#?}", result.file_name);
            println!("{} {} {}", &result.content[0..start_index], &self.query[..].bold(), &result.content[end_index..]);
        }
    }
    pub fn recurse_and_populate<'a>(&mut self, term: &'a str, dir: &Path) -> Result<Duration, io::Error>{
        let start = Instant::now();
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let path = entry?.path();
                if path.is_dir() {
                    self.recurse_and_populate(term, &path)?;
                } else {
                    self.search_in_file(term, &path).unwrap_or_else(|error| {
                        eprintln!("There was an error in reading file to string : {}", error);
                    });
                }
            }
        } else {
            self.search_in_file(term, dir).unwrap_or_else(|error| {
                eprintln!("There was an error in reading file to string : {}", error);
            });
        }
        Ok(start.elapsed())
    }
    fn search_in_file<'a>(&mut self, term: &'a str, file_name: &Path) -> Result<(), io::Error>{
        let content = fs::read_to_string(file_name)?;
        if content.contains(term) {
            self.results.push(SearchResult { content, file_name : Box::new(file_name.to_owned()) });
        }
        Ok(())
    }
}
