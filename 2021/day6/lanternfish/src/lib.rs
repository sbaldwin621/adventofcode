use std::collections::HashMap;

use thiserror::Error;

use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<u64, ApplicationError> {
    let filename = config.filename;
    let generations = config.generations;

    let file_contents = std::fs::read_to_string(filename)?;
    let fishes: Vec<u64> = file_contents.split(",").map(|s| s.parse::<u64>().unwrap()).collect();
    
    let count = solve(fishes, generations);

    Ok(count)
}

pub struct FishMap {
    map: HashMap<u64, u64>,
    count: u64
}

impl FishMap {
    pub fn new() -> FishMap {
        FishMap { map: HashMap::new(), count: 0 }
    }

    pub fn insert(&mut self, fish: u64, count: u64) {
        let new_count = self.map.get(&fish).unwrap_or(&0) + count;
        self.map.insert(fish, new_count);
        self.count = self.count + count;
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<u64, u64> {
        self.map.iter()
    }

    pub fn count(&self) -> u64 {
        self.count
    }
}

fn solve(starting_fishes: Vec<u64>, generations: u64) -> u64 {
    let mut fishes = FishMap::new();
    for fish in starting_fishes {
        fishes.insert(fish, 1);
    }

    let mut next_fishes = FishMap::new();

    for generation in 0..generations {
        for (&fish, &count) in fishes.iter() {
            if fish == 0 {  
                next_fishes.insert(6, count);
                next_fishes.insert(8, count);
            } else {
                next_fishes.insert(fish - 1, count);
            }
        }

        fishes = next_fishes;
        next_fishes = FishMap::new();
    }

    fishes.count()
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] std::io::Error)
}