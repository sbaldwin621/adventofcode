use std::fs;
use std::io;

use clap::Parser;
use memory_space::MemorySpace;
use memory_space::ParsePositionError;
use memory_space::PuzzleInput;
use thiserror::Error;

mod memory_space;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf,
    memory_space_size: usize,
    num_bytes: usize
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;
    
    let puzzle_input = fs::read_to_string(filename)?;
    let puzzle_input: PuzzleInput = puzzle_input.parse()?;

    let memory_space = puzzle_input.to_memory_space(options.memory_space_size, options.num_bytes);
    
    let result = match options.part {
        1 => run_part1(&memory_space),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(memory_space: &MemorySpace) -> Result<usize, ApplicationError> {
    if let Some(solution) = memory_space.solve() {
        Ok(solution)
    } else {
        Err(ApplicationError::CouldntFindSolution)
    }
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
    CouldntParseInput(#[from] ParsePositionError),
    #[error("couldn't find solution")]
    CouldntFindSolution
}