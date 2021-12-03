use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;
use parser::parse_puzzle_input;
use puzzle::Submarine;
use thiserror::Error;

use config::Config;

pub mod config;
pub mod parser;
pub mod puzzle;

pub fn run<'a>(config: Config) -> Result<i64, ApplicationError> {
    let filename = config.filename;

    let file_contents = read_to_string(filename)?;
    
    let (_, puzzle_input) = parse_puzzle_input(&file_contents)
        .map_err(|e| ApplicationError::CouldntParse(e.to_string().clone()))?;

    let mut submarine = Submarine::new();

    for instruction in puzzle_input.instructions().iter() {
        submarine.go(instruction);
    }
      
    Ok(submarine.value())
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

    #[error("couldn't parse puzzle input: {0}")]
    CouldntParse(String)
}
