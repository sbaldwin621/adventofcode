use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<u32, ApplicationError> {
    let filename = options.filename;

    let lines = read_lines(filename)?;
    for _line in lines {
        // do something with lines
    }

    Ok(0)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}