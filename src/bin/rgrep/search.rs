use std::{
    fs,
    path::Path,
};

pub struct Search {
    results: Vec<String>,
}

impl Search {

    pub fn new() -> Search {
        Search {results : Vec::new()}
    }
    pub fn print_results(&self) -> () {
        for result in self.results.iter() {
            println!("{}", result);
        }
    }
    pub fn recurse_and_return<'a>(&mut self, term: &'a str, dir: &Path) {
        if dir.is_dir() {
            for entry in fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    self.recurse_and_return(term, &path);
                } else {
                    self.search_in_file(term, &path);
                }
            }
        } else {
            self.search_in_file(term, dir);
        }
    }
    fn search_in_file<'a>(&mut self, term: &'a str, file_name: &Path) {
        let file_content = fs::read_to_string(file_name).unwrap();
        if file_content.contains(term) {
            self.results.push(file_content);
        }
    }
}
