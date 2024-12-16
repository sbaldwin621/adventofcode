use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use garden::{GardenMap, RegionFinder};
use thiserror::Error;

mod garden;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;

    let mut lines = vec![];
    for line in read_lines(filename)? {
        let line = line?;
        lines.push(line);
    }
    
    let garden_map = GardenMap::from_lines(&lines);    

    let result = match options.part {
        1 => run_part1(&garden_map),
        2 => run_part2(&garden_map),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(garden_map: &GardenMap) -> Result<usize, ApplicationError> {
    let mut region_finder = RegionFinder::new(&garden_map);
    let price = region_finder.calculate_fence_prices();

    Ok(price.total_price())
}

fn run_part2(garden_map: &GardenMap) -> Result<usize, ApplicationError> {
    let mut region_finder = RegionFinder::new(&garden_map);
    let price = region_finder.calculate_fence_prices();

    Ok(price.discount_price())
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