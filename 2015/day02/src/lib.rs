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

    Ok(puzzle_input.total_paper_needed().to_string())
}

fn run_part2(input: PathBuf) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;

    Ok(puzzle_input.total_ribbon_needed().to_string())
}

fn read_puzzle_input(filename: PathBuf) -> Result<PuzzleInput, ApplicationError> {
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
    CouldntParseInput(#[from] ParsePuzzleInputError)
}