use std::{path::Path, process};

use rtools::parser::parse_args;

mod search;
mod config;
fn main() {
    let grep_config : config::GrepConfig = parse_args();

    let mut new_search = search::Search::default();

    //safer to pass references to config fields than actual object as recursive definition internally cahanges the config fields. 
    let elapsed_time = new_search.recurse_and_populate(
        &grep_config.term[..],
        Path::new(&grep_config.path[..]),
    ).unwrap_or_else(|error| {
        eprintln!("There was an error in recursing through the directory specified : {}", error);
        process::exit(1)
        //so that print_results is not called with default empty string.
    });

    new_search.print_results();
    println!("Process completed in {:#?}", elapsed_time);
}
