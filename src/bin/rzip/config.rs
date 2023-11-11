use std::{collections::HashMap, process};
use itertools::Itertools;
use rtools::parser::Parsable;


pub struct ZipConfig {
    pub source : String,
    pub target :  String,
    //currently only --unzip which is not k-v pair, if no more plans for options, will restructure this (along with other non kv options)
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
        let unzip_option = match args.next() {
            Some(unzip) => unzip,
            None => "".to_string(),
        };
        // let options = args.tuples().collect();
        let mut options_hm = HashMap::new();
        options_hm.insert(unzip_option, "".to_string());

        Ok(Box::new(ZipConfig{source, target, options : options_hm }))
    }
}
