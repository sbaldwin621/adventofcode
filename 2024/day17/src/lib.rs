use std::fs;
use std::io;
use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use computer::search_for_quine;
use computer::DebuggerInfo;
use computer::Emulator;
use computer::ParseDebuggerInfoError;
use thiserror::Error;

mod computer;

#[derive(Parser)]
pub struct CliOptions {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Part1 {
        input: PathBuf
    },
    Part2 {
        input: PathBuf
    },
    RunProgram {
        input: PathBuf,
        register_a: String
    }
}

pub fn run(options: CliOptions) -> Result<(), ApplicationError> {
    match options.command {
        Commands::Part1 { input } => run_part1(input),
        Commands::Part2 { input } => run_part2(input),
        Commands::RunProgram { input, register_a } => run_program(input, register_a)
    }?;
    
    Ok(())
}

fn run_part1(input: PathBuf) -> Result<(), ApplicationError> {
    let puzzle_input = fs::read_to_string(input)?;
    let debugger_info: DebuggerInfo = puzzle_input.parse()?;

    let mut emulator = Emulator::from_debugger_info(&debugger_info);
    while emulator.step() { }

    let output = emulator.output_buffer();
    let output = output.iter().map(|v| v.to_string()).collect::<Vec<String>>();

    println!("{}", output.join(","));

    Ok(())
}

fn run_part2(input: PathBuf) -> Result<(), ApplicationError> {
    let puzzle_input = fs::read_to_string(input)?;
    let debugger_info: DebuggerInfo = puzzle_input.parse()?;

    if let Some(solution) = search_for_quine(&debugger_info) {
        println!("{}", solution);
        
        Ok(())
    } else {
        Err(ApplicationError::CouldntFindSolution)
    }
}

fn run_program(input: PathBuf, register_a: String) -> Result<(), ApplicationError> {
    let puzzle_input = fs::read_to_string(input)?;
    let debugger_info: DebuggerInfo = puzzle_input.parse()?;

    let register_a = permissive_parse(register_a);

    let mut emulator = Emulator::from_debugger_info(&debugger_info);
    *emulator.register_a_mut() = register_a;

    while emulator.step() { }

    let output = emulator.output_buffer();
    let output = output.iter().map(|v| v.to_string()).collect::<Vec<String>>();

    println!("{}", output.join(","));

    Ok(())
}

fn permissive_parse(n: String) -> usize {    
    if let Some(rest) = n.strip_prefix("0o") {
        usize::from_str_radix(&rest, 8).unwrap()
    } else {
        usize::from_str_radix(&n, 10).unwrap()
    }
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse debugger info: {0}")]
    CouldntParsePuzzleInput(#[from] ParseDebuggerInfoError),
    #[error("couldn't find solution")]
    CouldntFindSolution
}