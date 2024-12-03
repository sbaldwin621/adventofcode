use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<u32, ApplicationError> {
    let filename = options.filename;

    let mut left_ids = vec![];
    let mut right_ids = vec![];
    
    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;
        let mut split_iter = line.split_whitespace();
        let left_id = split_iter.next().ok_or(ApplicationError::InvalidLine())?.parse::<u32>()?;
        let right_id = split_iter.next().ok_or(ApplicationError::InvalidLine())?.parse::<u32>()?;
        
        left_ids.push(left_id);
        right_ids.push(right_id);
    }

    left_ids.sort();
    right_ids.sort();

    let mut sum = 0;
    for (a, b) in zip(left_ids, right_ids) {
        sum += a.abs_diff(b);
    }
    
    Ok(sum)
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
    #[error("invalid input line")]
    InvalidLine(),
    #[error("invalid int: {0}")]
    InvalidInteger(#[from] std::num::ParseIntError)
}