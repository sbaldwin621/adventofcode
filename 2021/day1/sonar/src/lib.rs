use std::error::Error;
use std::fmt::Display;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;

    let mut increase_count = 0;
    let mut previous_depth = u32::MAX;

    let lines = read_lines(filename)?;
    for line in lines {
        let depth = line?.parse::<u32>()?;
        if depth > previous_depth {
            increase_count += 1;
        }

        previous_depth = depth;
    }

    Ok(increase_count)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub enum ApplicationError {
    AnError
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ApplicationError::AnError => "an error occurred"  
        })
    }
}

impl Error for ApplicationError { }
