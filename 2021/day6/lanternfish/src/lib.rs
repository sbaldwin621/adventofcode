use thiserror::Error;

use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<usize, ApplicationError> {
    let filename = config.filename;
    let generations = config.generations;

    let file_contents = std::fs::read_to_string(filename)?;
    let fishes: Vec<u64> = file_contents.split(",").map(|s| s.parse::<u64>().unwrap()).collect();
    
    let count = solve(fishes, generations);

    Ok(count)
}

fn solve(fishes: Vec<u64>, generations: u64) -> usize {
    let mut fishes = fishes;
    let mut next_fishes = vec![];

    for _ in 0..generations {
        for &fish in fishes.iter() {
            if fish == 0 {
                next_fishes.push(6);
                next_fishes.push(8);
            } else {
                next_fishes.push(fish - 1);
            }
        }

        fishes = next_fishes;
        next_fishes = vec![];
    }

    fishes.len()
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] std::io::Error)
}