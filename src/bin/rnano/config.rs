use rtools::parser::Parsable;

pub struct EditorConfig {
    pub file_name : String
}


impl Parsable for EditorConfig {
    fn build(mut args : impl Iterator<Item=String>) -> Result<Box<Self>, &'static str> {
        args.next().ok_or("There was an error in parsing arguments")?;
        let file_name = args.next().ok_or("There was an error in parsing arguments")?;
        Ok(Box::new(EditorConfig { file_name }))
    }
}