use std::{
    fs::File,
    io::{self, copy, BufReader},
    time::{Instant, Duration},
};

use flate2::{write::GzEncoder, Compression};

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
