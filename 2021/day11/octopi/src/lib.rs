use std::collections::btree_map::IterMut;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead};

use clap::Parser;
use thiserror::Error;

use octopi::{OctopiGrid, ParseOctopiGridError};

pub mod octopi;

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf,
    iterations: u64
}

pub fn run(options: CliOptions) -> Result<u64, ApplicationError> {
    let filename = options.filename;
    let iterations = options.iterations;

    let contents = read_to_string(filename)?;
    let mut grid = contents.parse::<OctopiGrid>()?;
    println!("{}", grid);

    let mut total_flashes = 0;
    for _ in 0..iterations {
        total_flashes += grid.step();
        println!("{}", grid);
    }

    Ok(total_flashes)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse puzzle input: {0}")]
    CouldntParseInput(#[from] ParseOctopiGridError)
}