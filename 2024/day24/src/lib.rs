use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io;
use std::io::LineWriter;
use std::io::Write;
use std::path::PathBuf;

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
        input: PathBuf
    },
    Part2 {
        input: PathBuf
    },
    OutputCsv {
        input: PathBuf,
        output: PathBuf
    },
    FindConnections {
        input: PathBuf,
        target_wire: String
    }
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let result = match options.command {
        Commands::Part1 { input } => run_part1(input),
        Commands::Part2 { input } => run_part2(input),
        Commands::OutputCsv { input, output } => output_csv(input, output),
        Commands::FindConnections { input, target_wire } => find_connections(input, &target_wire)
    }?;

    Ok(result.to_string())
}

fn run_part1(input: PathBuf) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;
    let mut device = puzzle_input.into_device();
    
    Ok(device.solve().to_usize().to_string())
}

fn run_part2(input: PathBuf) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;
    let mut device = puzzle_input.into_device();
    
    let expected_output = device.expected_output();
    let actual_output = device.solve();

    let difference = expected_output.difference(&actual_output);
    let wrong_wires = difference.trues();

    println!("incorrect wires: {}", wrong_wires.join(", "));

    todo!()
}

fn output_csv(input: PathBuf, output: PathBuf) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;
    let mut device = puzzle_input.to_device();

    let expected_output = device.expected_output();
    let actual_output = device.solve();

    let difference = expected_output.difference(&actual_output);
    
    let mut nodes_to_include = HashSet::new();
    for wire in difference.trues() {
        let connections = device.find_connections(&wire);
        nodes_to_include.extend(connections);
    }

    let file = File::create(output)?;
    let mut file = LineWriter::new(file);

    writeln!(file, "source,target")?;
    
    for (n, gate) in puzzle_input.gates().iter().enumerate() {
        if nodes_to_include.contains(gate.left_input()) &&
            nodes_to_include.contains(gate.right_input()) &&
            nodes_to_include.contains(gate.output()) {
                let gate_name = format!("{:?}({})", gate.operation(), n);
                writeln!(file, "{},{}", gate.left_input(), gate_name)?;
                writeln!(file, "{},{}", gate.right_input(), gate_name)?;
                writeln!(file, "{},{}", gate_name, gate.output())?;
        }
    }

    Ok("".to_string())
}

fn find_connections(input: PathBuf, target_wire: &str) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;
    let device = puzzle_input.into_device();

    let connections = device.find_connections(target_wire);

    println!("{:?} ({})", connections, connections.len());

    Ok("".to_string())
}

fn read_puzzle_input(filename: PathBuf) -> Result<PuzzleInput, ApplicationError> {
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