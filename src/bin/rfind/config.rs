use rtools::parser::Parsable;
use std::process;
pub struct FindConfig {
    pub directory: String,
    pub file_name: String,
}


impl Parsable for FindConfig {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Box<Self>, &'static str> {
        args.next().unwrap_or_else(|| {
            eprintln!("There was an error in parsing arguments");
            process::exit(1);
        });
        let directory = args.next().unwrap_or_else(|| {
            eprintln!("There was an error in parsing arguments");
            process::exit(1);
        });
        let file_name = args.next().unwrap_or_else(|| {
            eprintln!("There was an error in parsing arguments");
            process::exit(1);
        });

        Ok(Box::new(FindConfig {
            directory,
            file_name,
        }))
    }
}
