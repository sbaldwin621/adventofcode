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
    
    let mut a;
    let mut b = u32::MAX;
    let mut c = u32::MAX;
    let mut d = u32::MAX;
    
    let lines = read_lines(filename)?;
    for line in lines {
        let depth = line?.parse::<u32>()?;
        
        a = b;
        b = c;
        c = d;
        d = depth;

        let window_one = a.saturating_add(b).saturating_add(c);
        let window_two = b.saturating_add(c).saturating_add(d);

        if window_two > window_one {
            increase_count += 1;
        }
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
