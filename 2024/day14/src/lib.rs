use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::{Parser, ValueEnum};
use thiserror::Error;

mod robots;
use robots::{ParseRobotError, Robot, RobotSimulation};

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf,
    arena_type: ArenaType
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum ArenaType {
    Example,
    Full
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;

    let mut robots = vec![];

    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;

        let robot: Robot = line.parse()?;
        robots.push(robot);
    }

    let arena_size: (i64, i64) = match options.arena_type {
        ArenaType::Example => (11, 7),
        ArenaType::Full => (101, 103)
    };

    let simulation = RobotSimulation::new(robots, arena_size);

    let result = match options.part {
        1 => run_part1(&simulation),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(simulation: &RobotSimulation) -> Result<u32, ApplicationError> {
    todo!()
}

fn run_part2() -> Result<u32, ApplicationError> {
    todo!()
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
    #[error("couldn't parse robot robot: {0}")]
    CouldntParseRobot(#[from] ParseRobotError)
}