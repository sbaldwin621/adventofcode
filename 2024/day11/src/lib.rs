use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use stones::StoneLine;
use thiserror::Error;

mod stones;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;

    let contents = fs::read_to_string(filename)?;

    let mut stone_line: StoneLine = contents.parse().unwrap();

    let result = match options.part {
        1 => run_part1(&mut stone_line),
        2 => run_part2(&mut stone_line),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(stone_line: &mut StoneLine) -> Result<usize, ApplicationError> {
    for _ in 0..25 {
        stone_line.blink();
    }

    Ok(stone_line.score())
}

fn run_part2(stone_line: &mut StoneLine) -> Result<usize, ApplicationError> {
    for _ in 0..75 {
        stone_line.blink();
    }

    Ok(stone_line.score())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}