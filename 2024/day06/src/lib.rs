use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use guard::{GuardSimulation, LabMap, LabMapRow, LabMapTile};
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
        2 => run_part2(&map),
        _ => Err(ApplicationError::UnknownPart)
    }
}

fn run_part1(map: &LabMap) -> Result<usize, ApplicationError> {
    let simulation = run_standard_simulation(map);

    Ok(simulation.visited_count())
}

fn run_part2(map: &LabMap) -> Result<usize, ApplicationError> {
    let mut total_count = 0;
    let mut success_count = 0;

    let initial_simulation = run_standard_simulation(map);

    for (x, y) in initial_simulation.locations_visited() {
        let x = *x;
        let y = *y;

        if let LabMapTile::Empty = map.tile_at(x, y) {
            total_count += 1;
            println!("running simulation {} for {}, {}", total_count, x, y);

            let mut simulation = GuardSimulation::new(&map, Some((x, y)));

            while simulation.step() {
                if simulation.loop_detected() {
                    success_count += 1;
                    break;
                }
            }
        }
    }

    Ok(success_count)
}

fn run_standard_simulation(map: &LabMap) -> GuardSimulation {
    let mut simulation = GuardSimulation::new(&map, None);

    while simulation.step() { }

    simulation
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