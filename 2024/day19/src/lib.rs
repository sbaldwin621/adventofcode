use std::fs;
use std::io;

use clap::Parser;
use thiserror::Error;
use towels::ParsePuzzleError;
use towels::PuzzleInput;
use towels::TowelSolver;

mod towels;

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
        1 => run_part1(&puzzle_input),
        2 => run_part2(&puzzle_input),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(puzzle_input: &PuzzleInput) -> Result<usize, ApplicationError> {
    let mut solver = puzzle_input.to_solver();

    let completed_orders = solver.solve();
    let count = completed_orders.iter().filter(|(_, count)| *count > 0).count();

    Ok(count)
}

fn run_part2(puzzle_input: &PuzzleInput) -> Result<usize, ApplicationError> {
    let mut solver = puzzle_input.to_solver();

    let completed_orders = solver.solve();
    let total_unique_combinations = completed_orders.iter().map(|(_, count)| *count).sum();
    
    Ok(total_unique_combinations)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse puzzle input: {0}")]
    CouldntParsePuzzleInput(#[from] ParsePuzzleError)
}