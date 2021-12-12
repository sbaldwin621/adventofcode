use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use map::MapBuilder;
use thiserror::Error;

pub mod map;

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<u64, ApplicationError> {
    let filename = options.filename;

    let mut map_builder = MapBuilder::new();

    let lines = read_lines(filename)?;
    for line in lines {
        map_builder.add_line(&line?);
    }
    
    let map = map_builder.to_map();

    Ok(map.calculate_basin_score())
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