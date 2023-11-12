use rtools::parser::Parsable;

pub struct ManConfig {
    pub command : String
}

impl Parsable for ManConfig {
    fn build(mut args : impl Iterator<Item=String>) -> Result<Box<Self>, &'static str> {
        args.next().ok_or("There was an error in parsing arguments")?;
        let command = args.next().unwrap_or_else(|| {
            String::from("")
        });
        Ok(Box::new(ManConfig { command }))
    }
}