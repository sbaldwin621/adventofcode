use std::collections::VecDeque;
use std::error::Error;
use std::fmt::Display;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;
    let window_size = 3;

    let mut increase_count = 0;
    
    let mut window = VecDeque::with_capacity(window_size);
    let mut previous_total = u32::MAX;
    let mut current_total = 0;

    let lines = read_lines(filename)?;
    for line in lines {
        let depth = line?.parse::<u32>()?;
        
        current_total += depth;
        window.push_back(depth);

        if window.len() > window_size {
            if let Some(popped) = window.pop_front() {
                current_total -= popped;

                if current_total > previous_total {
                    increase_count += 1;
                }
            }
        }
        
        previous_total = current_total;
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
