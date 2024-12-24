use std::fs;
use std::io;

use clap::Parser;
use network_map::NetworkMap;
use network_map::ParseNetworkMapError;
use thiserror::Error;

mod network_map;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;
    let puzzle_input = fs::read_to_string(filename)?;
    let network_map: NetworkMap = puzzle_input.parse()?;
    
    let result = match options.part {
        1 => run_part1(&network_map),
        2 => run_part2(&network_map),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(network_map: &NetworkMap) -> Result<usize, ApplicationError> {
    let clusters = network_map.clusters_of_three();

    let t_count = clusters.iter()
        .filter(|c| c.iter().any(|n| n.starts_with('t')))
        .inspect(|[a,b,c]| println!("{},{},{}", a, b, c))
        .count();

    Ok(t_count)
}

fn run_part2(network_map: &NetworkMap) -> Result<usize, ApplicationError> {
    let clusters = network_map.clusters();

    todo!()
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse puzzle input: {0}")]
    CouldntParseInput(#[from] ParseNetworkMapError)
}