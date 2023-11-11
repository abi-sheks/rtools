extern crate flate2;

use rtools::parser::parse_args;
use std::fs::File;
use std::io::{BufReader, copy};
use flate2::Compression;
use flate2::write::GzEncoder;
use std::time::Instant;

fn main () {
    let (command_name, args) = parse_args();
    
    let mut in_file = BufReader::new(File::open(args.get("--source").unwrap()).unwrap());
    let out_file = File::create(args.get("--target").unwrap()).unwrap();
    let mut encoder = GzEncoder::new(out_file, Compression::default());
    let start = Instant::now();
    copy(&mut in_file, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!("The file has been compressed in {:#?}.", start.elapsed());


}