use std::fs;
use std::io;

use clap::Parser;
use thiserror::Error;

mod maze;
use maze::Maze;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;
    let puzzle_input = fs::read_to_string(filename)?;

    let maze: Maze = puzzle_input.parse().unwrap();

    maze.print();
    
    let result = match options.part {
        1 => run_part1(),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1() -> Result<u32, ApplicationError> {
    todo!()
}

fn run_part2() -> Result<u32, ApplicationError> {
    todo!()
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}