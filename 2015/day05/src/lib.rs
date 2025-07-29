use std::fs;
use std::io;
use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use thiserror::Error;

use crate::puzzle::{PuzzleInput};

mod puzzle;

#[derive(Parser)]
pub struct CliOptions {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Part1 {
        input: PathBuf
    },
    Part2 {
        input: PathBuf
    }
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let result = match options.command {
        Commands::Part1 { input } => run_part1(input),
        Commands::Part2 { input } => run_part2(input)
    }?;

    Ok(result.to_string())
}

fn run_part1(input: PathBuf) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;
    let nice_count = puzzle_input.part1_nice_strings().count();

    Ok(nice_count.to_string())
}

fn run_part2(input: PathBuf) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;
    let nice_count = puzzle_input.part2_nice_strings().count();
    
    Ok(nice_count.to_string())
}

fn read_puzzle_input(filename: PathBuf) -> Result<PuzzleInput, ApplicationError> {
    let puzzle_input = fs::read_to_string(filename)?;
    let puzzle_input: PuzzleInput = puzzle_input.parse().unwrap();

    Ok(puzzle_input)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}