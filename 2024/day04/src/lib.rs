use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use cross_search::CrossSearch;
use thiserror::Error;
use crate::word_search::WordSearch;

mod word_search;
mod cross_search;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<usize, ApplicationError> {
    let filename = options.filename;

    let lines = read_lines(filename)?;
    let mut lines_vec = vec![];    
    
    for line in lines {
        lines_vec.push(line?);
    }

    let row_size = lines_vec[0].len();
    let input = lines_vec.join("");

    let word_search = WordSearch::new(input.clone(), row_size);
    let cross_search = CrossSearch::new(input, row_size);

    match options.part {
        1 => run_part1(word_search),
        2 => run_part2(cross_search),
        _ => Err(ApplicationError::UnknownPart)
    }
}

fn run_part1(word_search: WordSearch) -> Result<usize, ApplicationError> {
    Ok(word_search.search())
}

fn run_part2(cross_search: CrossSearch) -> Result<usize, ApplicationError> {
    Ok(cross_search.search())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}