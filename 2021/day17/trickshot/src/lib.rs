use clap::Parser;
use thiserror::Error;

use crate::simulation::{Simulator, SimulationResult};

pub mod simulation;

#[derive(Parser)]
pub struct CliOptions {
    x1: isize,
    x2: isize,
    y1: isize,
    y2: isize
}

pub fn run(options: CliOptions) -> Result<u32, ApplicationError> {
    let CliOptions {x1, x2, y1, y2} = options;
    
    let simulator = Simulator::new((x1, x2, y1, y2));

    let mut count = 0;
    let mut best_starting_velocity = (0, 0);
    let mut overall_highest_y: isize = 0;

    for x in 0..=x2 {
        for y in -1000..1000 {
            let result = simulator.simulate((x, y));
            if let SimulationResult::Hit(highest) = result {
                count += 1;

                println!("hit ({}, {}) -> {}", x, y, highest);
                
                if highest > overall_highest_y {
                    overall_highest_y = highest;
                    best_starting_velocity = (x, y);
                }
            } else if let simulation::SimulationResult::Missed(furthest) = result {
                
            }
        }
    }

    println!("total: {}. best: {:?} -> {}", count, best_starting_velocity, overall_highest_y);

    Ok(0)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] std::io::Error)
}