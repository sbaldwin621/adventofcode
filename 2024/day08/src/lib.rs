use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use city_map::{CityMap, CityMapBuilder};
use clap::Parser;
use thiserror::Error;

mod city_map;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;

    let mut city_map_builder = CityMapBuilder::new();

    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;
        city_map_builder.add_line(&line);
    }

    let city_map = city_map_builder.into_city_map();
    
    let result = match options.part {
        1 => run_part1(city_map),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(city_map: CityMap) -> Result<u32, ApplicationError> {
    Ok(city_map.count_antinodes_within_map())
}

fn run_part2() -> Result<u32, ApplicationError> {
    todo!()
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