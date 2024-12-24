use std::fs;
use std::io;
use std::num::ParseIntError;

use clap::Parser;
use secret::MarketSimulator;
use secret::SecretGenerator;
use thiserror::Error;

mod secret;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;
    let puzzle_input = read_puzzle_input(filename)?;
    
    let result = match options.part {
        1 => run_part1(puzzle_input),
        2 => run_part2(puzzle_input),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn read_puzzle_input(filename: std::path::PathBuf) -> Result<Vec<u64>, ApplicationError> {
    let puzzle_input: Result<Vec<u64>, _> = fs::read_to_string(filename)?
        .lines()
        .map(|line| line.parse::<u64>())
        .collect();

    Ok(puzzle_input?)
}

fn run_part1(puzzle_input: Vec<u64>) -> Result<u64, ApplicationError> {
    let mut secret_generator = SecretGenerator::new(puzzle_input);

    for n in 0..2000 {
        secret_generator.step();
    }
    
    Ok(secret_generator.sum())
}

fn run_part2(puzzle_input: Vec<u64>) -> Result<u64, ApplicationError> {
    let market = MarketSimulator::new();
    
    let history = market.simulate(puzzle_input, 2000);
    let bananas = history.find_best_bananas();
    let bananas: u64 = bananas.try_into().unwrap();

    Ok(bananas)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse puzzle input: {0}")]
    CouldntParseInput(#[from] ParseIntError)
}