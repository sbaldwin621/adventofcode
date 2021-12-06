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

    let solution = puzzle_input.solve();

    Ok(solution)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}