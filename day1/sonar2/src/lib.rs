use std::error::Error;
use std::fmt::Display;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;
    let window_size = 3 + 1;

    let mut increase_count = 0;
    
    let mut window = vec![u32::MAX; window_size];
    
    let lines = read_lines(filename)?;
    for line in lines {
        let depth = line?.parse::<u32>()?;
        
        let mut first_total = 0u32;

        for i in 0..window.len() - 1 {
            window[i] = window[i + 1];
            first_total = first_total.saturating_add(window[i]);
        }

        window[window_size - 1] = depth;

        println!("{:?}", window);

        let second_total = first_total.saturating_sub(window[0]).saturating_add(depth);

        if second_total > first_total {
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
