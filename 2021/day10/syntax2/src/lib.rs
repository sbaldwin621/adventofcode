use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use thiserror::Error;

pub mod parser;

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<u64, ApplicationError> {
    let filename = options.filename;

    let mut scores = vec![];

    let lines = read_lines(filename)?;
    for line in lines {
        let parse_result = parser::parse(line?);
        let score = parse_result.score();

        if score > 0 {
            scores.push(score);
        }
    }

    scores.sort();

    let middle_score = scores.get(scores.len() / 2).unwrap();

    Ok(*middle_score)
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