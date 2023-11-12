use std::{path::Path, time::{Instant, Duration}, io::{self, Error}, fs};



pub struct Find {
    //query stored as it is used in print_results
    results: Vec<String>,
}


impl Find {
    pub fn new() -> Find {
        Find {results : Vec::new()}
    }

    pub fn print_results(&self) {
        //bad paradigm, this should return something. Printing should happen in main
        //Will rectify both this and rgrep print_result functions by returning some struct.
        println!("There were {} instances", self.results.len());
        for result in self.results.iter() {
            println!("{}", result)
        }
    }

    pub fn recurse_and_find(&mut self, directory: &Path, file_name: &Path) -> Result<Duration, io::Error>{
        let start = Instant::now();
        if file_name.is_dir() {
            return Err(Error::new(io::ErrorKind::Other, "Please specify a proper file name"));
        }
        if directory.is_dir() {
            for entry in fs::read_dir(directory)? {
                let path = entry?.path();
                if path.is_dir() {
                    self.recurse_and_find(&path, file_name)?;
                } else {
                    //ouch
                    if path.to_str().unwrap().split("/").last().unwrap() == file_name.to_str().unwrap() {
                        self.results.push(path.to_str().unwrap().to_string());
                    }
                }
            }
        } else {
            return Err(Error::new(io::ErrorKind::Other, "Please specify a proper directory name"));
        }
        Ok(start.elapsed())

    }
}