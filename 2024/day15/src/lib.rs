use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use thiserror::Error;
use warehouse::{ParseWarehouseSimulationError, WarehouseSimulation, WarehouseSimulationSpec};

mod warehouse;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;
    
    let file_contents = read_to_string(filename)?;

    let spec: WarehouseSimulationSpec = file_contents.parse()?;

    let result = match options.part {
        1 => run_part1(&spec),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result.to_string())
}

fn run_part1(spec: &WarehouseSimulationSpec) -> Result<i32, ApplicationError> {
    let mut simulation = WarehouseSimulation::from_spec(spec);
    
    for instruction in spec.instructions().iter() {
        simulation.process_instruction(*instruction);
    }

    let score = simulation.score();

    Ok(score)
}

fn run_part2() -> Result<i32, ApplicationError> {
    todo!()
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse warehouse simulation spec: {0}")]
    CoulndParseInput(#[from] ParseWarehouseSimulationError)
}