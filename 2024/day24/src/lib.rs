use std::fs;
use std::io;

use clap::Parser;
use gates::ParsePuzzleInputError;
use gates::PuzzleInput;
use thiserror::Error;

mod gates;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;
    let puzzle_input = fs::read_to_string(filename)?;
    let puzzle_input: PuzzleInput = puzzle_input.parse()?;
    
    let result = match options.part {
        1 => run_part1(puzzle_input),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(puzzle_input: PuzzleInput) -> Result<usize, ApplicationError> {
    let mut device = puzzle_input.into_device();
    
    Ok(device.solve())
}

fn run_part2() -> Result<usize, ApplicationError> {
    todo!()
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse puzzle input: {0}")]
    CouldntParsePuzzleInput(#[from] ParsePuzzleInputError)
}