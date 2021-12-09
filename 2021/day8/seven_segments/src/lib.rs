use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<u32, ApplicationError> {
    let filename = options.filename;

    let mut total = 0;

    let lines = read_lines(filename)?;
    for line in lines {
        let split: Vec<String> = line?.split("|").take(2).map(|s| s.to_string()).collect();
        let digits: Vec<String> = split[1].split(" ").filter(|s| s.len() > 0).map(|s| s.to_string()).collect();
        
        for digit in digits {
            match digit.len() {
                2 => { 
                    // 1
                    total += 1;
                }
                4 => {
                    // 4
                    total += 1;
                }
                3 => {
                    // 7
                    total += 1;
                }
                7 => {
                    // 8
                    total += 1;
                }
                _ => { }   
            }
        }
    }

    Ok(total)
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