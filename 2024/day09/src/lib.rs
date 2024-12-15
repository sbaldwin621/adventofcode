use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use disk_map::{DiskMap, DiskMapParseError};
use thiserror::Error;

mod disk_map;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;

    let line = fs::read_to_string(filename)?;

    let mut disk_map: DiskMap = line.parse()?;

    let result = match options.part {
        1 => run_part1(&mut disk_map),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(disk_map: &mut DiskMap) -> Result<usize, ApplicationError> {
    disk_map.compact();
    
    Ok(disk_map.checksum())
}

fn run_part2() -> Result<usize, ApplicationError> {
    todo!()
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse puzzle input: {0}")]
    CouldntParseInput(#[from] DiskMapParseError)
}