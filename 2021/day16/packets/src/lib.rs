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
    let packet = parse_packet(&contents);
    
    println!("{:?}", packet);

    let result = packet.evaluate();

    Ok(result)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}