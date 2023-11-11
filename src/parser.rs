use std::{env, process};


pub trait Parsable {
    fn build(args : impl Iterator<Item=String>) -> Result<Box<Self>, &'static str>;
}


pub fn parse_args<T>() -> T
where
 T: Parsable
    {
    //iterator
    let args = env::args();
    let config : Box<T> = T::build(args).unwrap_or_else(|error| {
        eprintln!("There was an error in parsing the arguments : {}", error);
        process::exit(1);
    });
    *config
}
