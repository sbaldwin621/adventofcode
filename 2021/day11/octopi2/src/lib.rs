use std::collections::btree_map::IterMut;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead};

use clap::Parser;
use thiserror::Error;

use octopi::{OctopiGrid, ParseOctopiGridError};

pub mod octopi;

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<u64, ApplicationError> {
    let filename = options.filename;

    let contents = read_to_string(filename)?;
    let mut grid = contents.parse::<OctopiGrid>()?;
    println!("{}", grid);

    let len = grid.len();

    let mut n = 0;
    loop {
        n += 1;

        let flashes = grid.step();
        println!("{}", grid);

        if flashes == len.try_into().unwrap() {
            break;
        }
    }

    Ok(n)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse puzzle input: {0}")]
    CouldntParseInput(#[from] ParseOctopiGridError)
}