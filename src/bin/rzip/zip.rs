use std::{
    fs::File,
    io::{self, copy, BufReader, Write, Read},
    time::{Instant, Duration},
};

use flate2::{write::{GzEncoder, GzDecoder}, Compression};

use crate::config::ZipConfig;

pub fn zip(config: ZipConfig) -> Result<Duration, io::Error> {
    let mut in_file = BufReader::new(File::open(config.source)?);
    let out_file = File::create(config.target)?;
    let mut encoder = GzEncoder::new(out_file, Compression::default());
    let start = Instant::now();
    copy(&mut in_file, &mut encoder)?;
    encoder.finish()?;
    Ok(start.elapsed())
}

pub fn unzip(config : ZipConfig) -> Result<Duration, io::Error> {
    let mut binary_stream = Vec::new();
    let mut decoder = GzDecoder::new(binary_stream);

    let start = Instant::now();

    //gets bytes from file
    let in_file = File::open(config.source)?;
    let mut reader = BufReader::new(in_file);
    let mut buffer = Vec::new();
    // Read file into vector.
    reader.read_to_end(&mut buffer)?;

    //writes decoded bytes into writer
    decoder.write_all(&buffer)?;
    binary_stream = decoder.finish()?;

    //decoded bytes from writer are written into specified output file.
    let mut out_file = File::create(config.target)?;
    out_file.write(&binary_stream[..])?;
    let return_string = String::from_utf8(binary_stream).unwrap();
    println!("{}", return_string);
    Ok(start.elapsed())
}

