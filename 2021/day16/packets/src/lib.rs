use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use itertools::Itertools;
use thiserror::Error;

use crate::{
    parser::parse_packet
};

pub mod bits;
pub mod parser;

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<usize, ApplicationError> {
    let filename = options.filename;

    let contents = read_to_string(filename)?;
    let binary = get_binary(contents);
    let slice = &binary[..];
    let (_, packet) = parse_packet(slice).unwrap();
    
    println!("{:?}", packet);

    let version_sum = packet.version_sum();

    Ok(version_sum)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_binary(s: String) -> Vec<u8> {
    let mut result = vec![];

    for chunk in &s.chars().chunks(2) {
        let mut s = String::new();
        for c in chunk {
            s.push(c);
        }
        
        let n = u8::from_str_radix(&s, 16).unwrap();
        result.push(n);
    }

    result
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}