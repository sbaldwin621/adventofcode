use std::fs::{File};
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;
use decoder::Decoder;
use thiserror::Error;

use config::Config;

pub mod config;
pub mod decoder;

pub fn run(config: Config) -> Result<u64, ApplicationError> {
    let filename = config.filename;

    let mut decoder = Decoder::new();

    let lines = read_lines(filename)?;
    for line in lines {
        let value = u64::from_str_radix(&line?, 2)?;
        decoder.ingest(value);
    }

    Ok(decoder.calculate_power_consumption())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse line")]
    ParseError(#[from] ParseIntError)
}