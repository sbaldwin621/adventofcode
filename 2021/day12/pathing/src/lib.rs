use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use thiserror::Error;

pub mod graph;
use graph::{
    Cave,
    CaveSystem
};

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<usize, ApplicationError> {
    let filename = options.filename;

    let mut cave_system = CaveSystem::new();

    let lines = read_lines(filename)?;
    for line in lines {
        let s: Vec<String> = line?.split('-')
            .take(2)
            .map(|s| s.to_string())
            .collect();

        let from = &s[0];
        let to = &s[1];
        
        cave_system.add_connection(from, to);
    }

    let paths = cave_system.paths();

    println!("{:?}", cave_system);
    println!("{:?}", paths);

    Ok(paths.len())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}