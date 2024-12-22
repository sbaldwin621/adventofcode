use std::fs;
use std::io;

use clap::Parser;
use keypad::PuzzleSolver;
use thiserror::Error;

mod keypad;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;
    let codes: Vec<String> = fs::read_to_string(filename)?
        .lines()
        .map(|s| s.trim().to_string())
        .collect();

    let result = match options.part {
        1 => run_part1(codes),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(codes: Vec<String>) -> Result<u32, ApplicationError> {
    let puzzle_solver = PuzzleSolver::new();

    for code in codes {
        puzzle_solver.solve(&code);
    }

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