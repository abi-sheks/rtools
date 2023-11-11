use std::{collections::HashMap, process};

use itertools::Itertools;
use rtools::parser::Parsable;
pub struct GrepConfig {
    pub term: String,
    pub path: String,
    pub options: HashMap<String, String>,
}

impl Parsable for GrepConfig {
    fn build(mut args: impl Iterator<Item=String>) -> Result<Box<Self>, &'static str> {
        args.next().unwrap_or_else(|| {
            eprintln!("There was an error in parsing arguments");
            process::exit(1);
        });
        let term = args.next().unwrap_or_else(|| {
            eprintln!("There was an error in parsing arguments");
            process::exit(1);
        });
        let path = args.next().unwrap_or_else(|| {
            eprintln!("There was an error in parsing arguments");
            process::exit(1);
        });
        let options = args.tuples().collect();

        Ok(Box::new(GrepConfig { term, path, options}))
    }
}
