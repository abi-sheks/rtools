use rtools::parser::Parsable;

pub struct SortConfig {
    pub file_name: String,
}

impl Parsable for SortConfig {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Box<Self>, &'static str> {
        args.next().ok_or("Please format the command properly")?;
        let file_name = args.next().ok_or("There was an error in parsing arguments")?;

        Ok(Box::new(SortConfig { file_name }))
    }
}
