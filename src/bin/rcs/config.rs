use rtools::parser::Parsable;
pub struct RCSConfig {
    pub command_name: String,
    pub arguments: Vec<String>,
}

impl Parsable for RCSConfig {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Box<Self>, &'static str> {
        args.next()
            .ok_or("There was an error in parsing arguments")?;
        let command_name = args
            .next()
            .ok_or("There was an error in parsing arguments")?;
        let mut arguments: Vec<String> = Vec::new();
        if command_name == "add" {
            let file_name = args
            .next()
            .ok_or("There was an error in parsing arguments")?;
            arguments.push(file_name);
        }
        if command_name == "commit" {
            let message_flag = args
            .next()
            .ok_or("There was an error in parsing arguments")?;
            let message = args
            .next()
            .ok_or("There was an error in parsing arguments")?;
            arguments.push(message_flag);
            arguments.push(message);
        }
        if command_name == "log" {
            if args.next().is_some() {
                return Err("There was an error in the format of the command.");
            }
         }
        Ok(Box::new(RCSConfig {
            command_name,
            arguments,
        }))
    }
}
