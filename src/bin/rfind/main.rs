use std::{path::Path, process};

use rtools::parser::parse_args;

mod find;
mod config;
fn main () {
    let find_config : config::FindConfig = parse_args();
    let mut find_results = find::Find::new();
    let elapsed_time = find_results.recurse_and_find(
        Path::new(&find_config.directory[..]),
        Path::new(&find_config.file_name[..]),
    ).unwrap_or_else(|error| {
        eprintln!("There was an error in recursing through the directory specified : {}", error);
        process::exit(1)
        //so that print_results is not called with default empty string.
    });

    find_results.print_results();
    println!("Process completed in {:#?}", elapsed_time);

}