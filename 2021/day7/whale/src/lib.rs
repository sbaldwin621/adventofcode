use clap::Parser;
use itertools::Itertools;
use thiserror::Error;

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<u64, ApplicationError> {
    let filename = options.filename;

    let contents = std::fs::read_to_string(filename)?;
    let crabs: Vec<u64> = contents.split(",").map(|s| s.parse::<u64>().unwrap()).collect();
    
    let (best_position, fuel_used) = solve(crabs);

    Ok(fuel_used)
}

fn solve(crabs: Vec<u64>) -> (u64, u64) {
    let positions = crabs.iter()
        .sorted()
        .unique();

    let mut best_position = u64::MAX;
    let mut minimum = u64::MAX;

    for &position in positions {
        let mut total_fuel_used = 0;

        for &crab in crabs.iter() {
            let difference = if crab > position {
                crab - position
            } else {
                position - crab
            };

            total_fuel_used += difference;
        }

        if total_fuel_used < minimum {
            best_position = position;
            minimum = total_fuel_used;
        }
    }

    (best_position, minimum)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] std::io::Error)
}