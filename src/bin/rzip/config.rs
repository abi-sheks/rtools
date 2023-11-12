use std::collections::HashMap;
use rtools::parser::Parsable;


pub struct ZipConfig {
    pub source : String,
    pub target :  String,
    //currently only --unzip which is not k-v pair, if no more plans for options, will restructure this (along with other non kv options)
    pub options : HashMap<String, String>,
}

impl Parsable for ZipConfig {
    fn build(mut args : impl Iterator<Item=String>) -> Result<Box<Self>, &'static str> {
        args.next().ok_or("There was an error in parsing arguments")?;
        let source = args.next().ok_or("There was an error in parsing arguments")?;
        let target = args.next().ok_or("There was an error in parsing arguments")?;
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
