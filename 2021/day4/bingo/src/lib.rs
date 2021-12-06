use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;
use bingo::{BingoBoard, PuzzleInput};
use thiserror::Error;

use config::Config;

pub mod bingo;
pub mod config;

pub fn run(config: Config) -> Result<u64, ApplicationError> {
    let filename = config.filename;
    let mut puzzle_input = PuzzleInput::load_from_file(&filename).unwrap();

    let mut solutions = puzzle_input.solve();

    solutions.sort_by(|a, b| a.turns_required.partial_cmp(&b.turns_required).unwrap());

    let first = solutions.first().unwrap();
    let last = solutions.last().unwrap();

    println!("first: {:?}", first);
    println!("last: {:?}", last);

    Ok(0)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}