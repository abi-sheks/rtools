

use rtools::parser::parse_args;
mod config;


fn main() {
    let man_config : config::ManConfig = parse_args(); 
    match &man_config.command[..] {
        //basically user has typed rman only, so we provide help with rman
        "" => {
            println!("The rman tool can be used to view information for the other tools.\n
                      Usage : rman <command>");
        },  
        "rgrep" => {
            println!("The rgrep tool can be used to find matches for a search term in a particular directory\n
                      Usage : rgrep <search_term> <file/directory>
            ");
        },
        "rzip" => {
            println!("The rzip tool can be used to zip files in the gzip compression format and uncompress file data that is in gzip compression format. \n
            Usage : rzip <source> <target>\n
            Options : \n
            --unzip : Unzips source and writes to target.
            ")
        },
        "rfind" => {
            println!("The rfind tool can be used to find instances of a specified file in a folder, returning the full path of each instance.\n
            Usage : rfind <directory> <file_name>")
        }
        &_ => {
            eprintln!("Please enter a valid rtools command. (rgrep, rzip)");
        }
    }
}