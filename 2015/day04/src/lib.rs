use std::fs;
use std::io;
use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use thiserror::Error;

use crate::puzzle::{ParsePuzzleInputError, PuzzleInput};

mod puzzle;

#[derive(Parser)]
pub struct CliOptions {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Part1 {
        input: String
    },
    Part2 {
        input: String
    }
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let result = match options.command {
        Commands::Part1 { input } => run_part1(input),
        Commands::Part2 { input } => run_part2(input)
    }?;

    Ok(result.to_string())
}

fn run_part1(input: String) -> Result<String, ApplicationError> {
    let puzzle_input: PuzzleInput = input.parse().unwrap();

    Ok(puzzle_input.mine(5).to_string())
}

fn run_part2(input: String) -> Result<String, ApplicationError> {   
    let puzzle_input: PuzzleInput = input.parse().unwrap();

    Ok(puzzle_input.mine(6).to_string())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}