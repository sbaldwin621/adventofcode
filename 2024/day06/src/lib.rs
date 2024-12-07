use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use guard::{GuardSimulation, LabMap, LabMapRow};
use thiserror::Error;

mod guard;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<usize, ApplicationError> {
    let filename = options.filename;

    let mut rows = vec![];

    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;
        let map_row: LabMapRow = line.parse()?;
        rows.push(map_row);
    }

    let map = LabMap::new(rows);

    match options.part {
        1 => run_part1(&map),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }
}

fn run_part1(map: &LabMap) -> Result<usize, ApplicationError> {
    let mut simulation = GuardSimulation::new(&map);

    while simulation.step() { }

    Ok(simulation.visited_count())
}

fn run_part2() -> Result<usize, ApplicationError> {
    !unimplemented!()
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
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse map row: {0}")]
    CouldntParseMapRow(#[from] guard::LabMapRowParseError)
}