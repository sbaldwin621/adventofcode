use core::num;
use std::fs;
use std::io;
use std::mem;

use clap::Parser;
use clap::Subcommand;
use memory_space::MemorySpace;
use memory_space::ParsePositionError;
use memory_space::PuzzleInput;
use thiserror::Error;

mod memory_space;

#[derive(Parser)]
pub struct CliOptions {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Part1 {
        filename: std::path::PathBuf,
        memory_space_size: usize,
        num_bytes: usize
    },
    Part2 {
        filename: std::path::PathBuf,
        memory_space_size: usize
    }
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let result = match options.command {
        Commands::Part1 { filename, memory_space_size, num_bytes} =>
            run_part1(filename, memory_space_size, num_bytes),
        Commands::Part2 { filename, memory_space_size} =>
            run_part2(filename, memory_space_size)
    }?;

    Ok(result)
}

fn run_part1(filename: std::path::PathBuf, memory_space_size: usize, num_bytes: usize) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(filename)?;
    
    let memory_space = puzzle_input.to_memory_space(memory_space_size, num_bytes);

    if let Some(solution) = memory_space.solve() {
        Ok(solution.to_string())
    } else {
        Err(ApplicationError::CouldntFindSolution)
    }
}

fn run_part2(filename: std::path::PathBuf, memory_space_size: usize) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(filename)?;

    for n in 1..puzzle_input.len() {
        println!("{}", n);

        let memory_space = puzzle_input.to_memory_space(memory_space_size, n);
        if let None = memory_space.solve() {
            if let Some(position) = puzzle_input.get(n - 1) {
                let x = position.x();
                let y = position.y();

                return Ok(format!("{},{}", x, y));
            }
        }
    }

    Err(ApplicationError::CouldntFindSolution)
}

fn read_puzzle_input(filename: std::path::PathBuf) -> Result<PuzzleInput, ApplicationError> {
    let puzzle_input = fs::read_to_string(filename)?;
    let puzzle_input: PuzzleInput = puzzle_input.parse()?;
    
    Ok(puzzle_input)
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