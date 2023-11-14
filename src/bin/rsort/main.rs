mod config;
mod sort;
use std::process;

use config::SortConfig;
use rtools::parser::parse_args;

fn main() {
    let sort_config : SortConfig = parse_args();
    let mut sort_results = sort::Sort::default();
    let elapsed_time = sort_results.sort_file(sort_config).unwrap_or_else(|error| {
        eprintln!("There was an error in the sorting operation : {}", error);
        process::exit(1);
    });

    sort_results.print_results();
    println!("Process completed in {:#?}", elapsed_time);



}