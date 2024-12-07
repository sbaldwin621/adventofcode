use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<u32, ApplicationError> {
    let filename = options.filename;

    let lines = read_lines(filename)?;
    for _line in lines {
        // do something with lines
    }

    match options.part {
        1 => run_part1(),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }
}

fn run_part1() -> Result<u32, ApplicationError> {
    !unimplemented!()
}

fn run_part2() -> Result<u32, ApplicationError> {
    !unimplemented!()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}