use std::fs;
use std::io;

use clap::Parser;
use maze::MazeSimulation;
use thiserror::Error;

mod maze;
use maze::Maze;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;
    let puzzle_input = fs::read_to_string(filename)?;

    let maze: Maze = puzzle_input.parse().unwrap();

    maze.print();
    
    let result = match options.part {
        1 => run_part1(&maze),
        2 => run_part2(&maze),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(maze: &Maze) -> Result<usize, ApplicationError> {
    let mut simulation = MazeSimulation::new(&maze);
    let solution = simulation.simulate().ok_or(ApplicationError::CouldntFindSolution)?;

    Ok(solution.best_score().try_into().unwrap())
}

fn run_part2(maze: &Maze) -> Result<usize, ApplicationError> {
    let mut simulation = MazeSimulation::new(&maze);
    let solution = simulation.simulate().ok_or(ApplicationError::CouldntFindSolution)?;
    
    Ok(solution.best_path_tile_count())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't find solution")]
    CouldntFindSolution
}