use std::collections::HashSet;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use thiserror::Error;

use paper::Paper;
use parser::parse_puzzle_input;

pub mod paper;
pub mod parser;
pub mod puzzle;

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<usize, ApplicationError> {
    let filename = options.filename;

    let contents = read_to_string(filename)?;
    let (_, puzzle_input) = parse_puzzle_input(&contents)
        .map_err(|e| ApplicationError::CouldntParse(e.to_string().clone()))?;

    let dot_set = HashSet::from_iter(puzzle_input.dots);

    let mut paper = Paper::new(dot_set);

    println!("{}", paper);

    for instruction in puzzle_input.instructions.iter() {
        match instruction {
            puzzle::Instruction::FoldUp(fold_y) => paper.fold_up(*fold_y),
            puzzle::Instruction::FoldLeft(fold_x) => paper.fold_left(*fold_x)
        }
        
        println!("{}", paper);
    }

    Ok(paper.len())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse puzzle input: {0}")]
    CouldntParse(String)
}