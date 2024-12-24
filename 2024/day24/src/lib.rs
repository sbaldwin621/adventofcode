use std::fs;
use std::fs::File;
use std::io;
use std::io::LineWriter;
use std::io::Write;

use clap::Parser;
use clap::Subcommand;
use gates::ParsePuzzleInputError;
use gates::PuzzleInput;
use thiserror::Error;

mod gates;

#[derive(Parser)]
pub struct CliOptions {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Part1 {
        input: std::path::PathBuf
    },
    Part2 {
        input: std::path::PathBuf
    },
    OutputCsv {
        input: std::path::PathBuf,
        output: std::path::PathBuf
    }
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let result = match options.command {
        Commands::Part1 { input } => run_part1(input),
        Commands::Part2 { input } => run_part2(input),
        Commands::OutputCsv { input, output } => output_csv(input, output),
    }?;

    Ok(result.to_string())
}

fn run_part1(input: std::path::PathBuf) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;
    let mut device = puzzle_input.into_device();
    
    Ok(device.solve().to_string())
}

fn run_part2(input: std::path::PathBuf) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;
    let mut device = puzzle_input.into_device();
    
    let expected_output = device.expected_output();
    let actual_output = device.solve();

    println!("{:b}", expected_output);
    println!("{:b}", actual_output);

    todo!()
}

fn output_csv(input: std::path::PathBuf, output: std::path::PathBuf) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;

    let file = File::create(output)?;
    let mut file = LineWriter::new(file);

    writeln!(file, "source,target")?;

    for gate in puzzle_input.gates() {
        writeln!(file, "{},{}", gate.left_input(), gate.output())?;
        writeln!(file, "{},{}", gate.right_input(), gate.output())?;
    }

    Ok("".to_string())
}

fn read_puzzle_input(filename: std::path::PathBuf) -> Result<PuzzleInput, ApplicationError> {
    let puzzle_input = fs::read_to_string(filename)?;
    let puzzle_input: PuzzleInput = puzzle_input.parse()?;

    Ok(puzzle_input)
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse puzzle input: {0}")]
    CouldntParsePuzzleInput(#[from] ParsePuzzleInputError)
}