// extern crate flate2;

use std::{process, time::Duration};

// use rtools::parser::parse_args;
// use std::fs::File;
// use std::io::{BufReader, copy};
// use flate2::Compression;
// use flate2::write::GzEncoder;
// use std::time::Instant;
use rtools::parser::parse_args;
mod config;
mod zip;

fn main() {
    //     let (command_name, args) = parse_args();
    let zip_config: config::ZipConfig = parse_args();
    let time_elapsed = if zip_config.options.contains_key("--unzip") {
        zip::unzip(zip_config).unwrap_or_else(|error| {
            eprintln!("There was an error in unzipping the file : {}", error);
            process::exit(1);
        })
    } else {
        zip::zip(zip_config).unwrap_or_else(|error| {
            eprintln!("There was an error in zipping the file : {}", error);
            process::exit(1);
        })
    };
    println!("The process was completed in {:#?}.", time_elapsed);
}
