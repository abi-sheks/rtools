use std::{collections::HashMap, process};
use itertools::Itertools;
use rtools::parser::Parsable;


pub struct ZipConfig {
    pub source : String,
    pub target :  String,
    pub options : HashMap<String, String>,
}

impl Parsable for ZipConfig {
    fn build(mut args : impl Iterator<Item=String>) -> Result<Box<Self>, &'static str> {
        args.next().unwrap_or_else(|| {
            eprintln!("There was an error in parsing arguments");
            process::exit(1);
        });
        let source = args.next().unwrap_or_else(|| {
            eprintln!("There was an error in parsing arguments");
            process::exit(1);
        });
        let target = args.next().unwrap_or_else(|| {
            eprintln!("There was an error in parsing arguments");
            process::exit(1);
        });
        let options = args.tuples().collect();

        Ok(Box::new(ZipConfig{source, target, options}))
    }
}
