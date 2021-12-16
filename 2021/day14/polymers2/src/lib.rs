use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use parser::parse_puzzle_input;
use thiserror::Error;

pub mod parser;
pub mod polymers;

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf,
    iterations: u64
}

pub fn run(options: CliOptions) -> Result<u64, ApplicationError> {
    let filename = options.filename;
    let iterations = options.iterations;

    let content = read_to_string(filename)?;
    let (_, puzzle_input) = parse_puzzle_input(&content).unwrap();

    let mut chain = puzzle_input.polymer_template.clone();

    for n in 0..iterations {
        chain.apply_rules(&puzzle_input.insertion_rules);
    }

    let score = chain.score();

    Ok(score)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}