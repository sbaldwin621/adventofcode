use std::fs;
use std::io;

use clap::Parser;
use computer::DebuggerInfo;
use computer::Emulator;
use computer::ParseDebuggerInfoError;
use thiserror::Error;

mod computer;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let filename = options.filename;
    let puzzle_input = fs::read_to_string(filename)?;

    let debugger_info: DebuggerInfo = puzzle_input.parse()?;
        
    let result = match options.part {
        1 => run_part1(&debugger_info),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }?;
    
    Ok(result)
}

fn run_part1(debugger_info: &DebuggerInfo) -> Result<String, ApplicationError> {
    let mut emulator = Emulator::from_debugger_info(debugger_info);
    while emulator.step() { }

    let output = emulator.output_buffer();
    let output = output.iter().map(|v| v.to_string()).collect::<Vec<String>>();
    
    Ok(output.join(","))
}

fn run_part2() -> Result<String, ApplicationError> {
    todo!()
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse debugger info: {0}")]
    CouldntParsePuzzleInput(#[from] ParseDebuggerInfoError)
}