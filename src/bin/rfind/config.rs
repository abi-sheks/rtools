use rtools::parser::Parsable;
pub struct FindConfig {
    pub directory: String,
    pub file_name: String,
}


impl Parsable for FindConfig {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Box<Self>, &'static str> {
        args.next().ok_or("There was an error in parsing arguments")?;
        let directory = args.next().ok_or("There was an error in parsing arguments")?;
        let file_name = args.next().ok_or("There was an error in parsing arguments")?;

        Ok(Box::new(FindConfig {
            directory,
            file_name,
        }))
    }
}
