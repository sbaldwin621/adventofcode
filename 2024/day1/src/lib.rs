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

    let mut left = Vec::new();
    let mut right = Vec::new();

    let lines = read_lines(filename)?;
    for line in lines {
        let line = line.unwrap();
        let mut columns_iter = line.split_whitespace();
        let column_one = columns_iter.next().unwrap();
        let column_two = columns_iter.next().unwrap();
        let column_one = column_one.parse::<u32>().unwrap();
        let column_two = column_two.parse::<u32>().unwrap();

        left.push(column_one);
        right.push(column_two);
    }

    left.sort();
    right.sort();

    let mut sum = 0;
    for (a, b) in zip(left, right) {
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
    CouldntReadInput(#[from] io::Error)
}