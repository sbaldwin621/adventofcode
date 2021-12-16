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

pub fn run(options: CliOptions) -> Result<usize, ApplicationError> {
    let filename = options.filename;
    let iterations = options.iterations;

    let content = read_to_string(filename)?;
    let (_, puzzle_input) = parse_puzzle_input(&content).unwrap();

    let mut chain = puzzle_input.polymer_template.clone();

    for _ in 0..iterations {
        chain.apply_rules(&puzzle_input.insertion_rules);
    }

    println!("{}", chain);

    let score = chain.score();

    Ok(score)
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