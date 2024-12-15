use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use thiserror::Error;

mod topographic_map;

use topographic_map::{TopographicMap};

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;
    
    let lines = read_lines(filename)?;
    let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();

    let map = TopographicMap::from_lines(&lines);

    let result = match options.part {
        1 => run_part1(&map),
        2 => run_part2(&map),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(map: &TopographicMap) -> Result<usize, ApplicationError> {
    let score = map.score();

    Ok(score)
}

fn run_part2(map: &TopographicMap) -> Result<usize, ApplicationError> {
    let rating = map.rating();

    Ok(rating)
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