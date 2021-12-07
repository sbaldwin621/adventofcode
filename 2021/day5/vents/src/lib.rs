use thiserror::Error;

pub mod config;
pub mod parser;
pub mod puzzle;

use crate::config::Config;
use crate::parser::{parse_puzzle_input};
use crate::puzzle::Solver;

pub fn run(config: Config) -> Result<u64, ApplicationError> {
    let filename = config.filename;

    let file_contents = std::fs::read_to_string(filename)?;
    
    let (_, puzzle_input) = parse_puzzle_input(&file_contents)
        .map_err(|e| ApplicationError::CouldntParse(e.to_string().clone()))?;
    
    let mut solver = Solver::new();
    for line_segment in puzzle_input.line_segments() {
        solver.ingest(&line_segment);
    }

    let overlaps = solver.count_overlaps();

    Ok(overlaps)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] std::io::Error),

    #[error("couldn't parse puzzle input: {0}")]
    CouldntParse(String)
}