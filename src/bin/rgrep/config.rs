use std::collections::HashMap;

use itertools::Itertools;
use rtools::parser::Parsable;
pub struct GrepConfig {
    pub term: String,
    pub path: String,
    pub options: HashMap<String, String>,
}

impl Parsable for GrepConfig {
    fn build(mut args: impl Iterator<Item=String>) -> Result<Box<Self>, &'static str> {
        args.next().ok_or("Please format the command properly")?;
        let term = args.next().ok_or("There was an error in parsing arguments")?;
        let path = args.next().ok_or("There was an error in parsing arguments")?;
        let options = args.tuples().collect();

        Ok(Box::new(GrepConfig { term, path, options}))
    }
}
