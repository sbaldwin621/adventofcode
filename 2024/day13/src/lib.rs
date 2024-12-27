use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use crane_game::CraneGameList;
use thiserror::Error;

mod crane_game;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;

    let contents = fs::read_to_string(filename)?;
    
    let crane_game_list: CraneGameList = contents.parse().unwrap();

    let result = match options.part {
        1 => run_part1(&crane_game_list),
        2 => run_part2(&crane_game_list),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(crane_game_list: &CraneGameList) -> Result<i64, ApplicationError> {
    let total_tokens = crane_game_list.solve(0);
    
    Ok(total_tokens)
}

fn run_part2(crane_game_list: &CraneGameList) -> Result<i64, ApplicationError> {
    let total_tokens = crane_game_list.solve(10000000000000);
    
    Ok(total_tokens)
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
    CouldntReadInput(#[from] io::Error)
}