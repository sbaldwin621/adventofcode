use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io;
use std::io::LineWriter;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use gates::{ParsePuzzleInputError, PuzzleInput, GateOperation};
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
    MapUpstream {
        input: PathBuf,
        target_wire: String,

        #[arg(long, default_value_t = usize::MAX)]
        depth: usize
    },
    MapDownstream {
        input: PathBuf,
        target_wire: String,

        #[arg(long, default_value_t = usize::MAX)]
        depth: usize
    }
}

pub fn run(options: CliOptions) -> Result<String, ApplicationError> {
    let result = match options.command {
        Commands::Part1 { input } => run_part1(input),
        Commands::Part2 { input } => run_part2(input),
        Commands::OutputCsv { input, output } => output_csv(input, output),
        Commands::MapUpstream { input, target_wire, depth } => map_upstream(input, &target_wire, depth),
        Commands::MapDownstream { input, target_wire, depth } => map_downstream(input, &target_wire, depth),
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

    let mut previous_carry = device.find_output("x00", "y00", GateOperation::And)
        .expect("puzzle input missing initial carry gate");

    for n in 1..=44 {
        println!("inspecting '{n}' adder (carry: {previous_carry})");

        let x = &format!("x{n:02}");
        let y = &format!("y{n:02}");
        let z = &format!("z{n:02}");

        let xy_xor = device.find_output(x, y, GateOperation::Xor)
            .expect("puzzle input missing input XOR gate");

        let xy_and = device.find_output(x, y, GateOperation::And)
            .expect("puzzle input missing input AND gate");

        if let Some(carry_xor) = device.find_output(previous_carry, xy_xor, GateOperation::Xor) {
            if carry_xor != z {
                panic!("{previous_carry} ^ {xy_xor} != {z}");
            }

            if let Some(carry_and) = device.find_output(previous_carry, xy_xor, GateOperation::And) {
                if let Some(next_carry) = device.find_output(carry_and, xy_and, GateOperation::Or) {
                    previous_carry = next_carry;
                } else {
                    // some error
                    todo!();
                }
            } else {
                // some error
                todo!();
            }
        } else {
            // some error
            todo!();
        }
    }

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
        let connections = device.find_upstream(&wire, usize::MAX);
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

fn map_upstream(input: PathBuf, target_wire: &str, depth: usize) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;
    let device = puzzle_input.into_device();

    let connections = device.find_upstream(target_wire, depth);
    let mut connections: Vec<_> = connections.iter().collect();
    connections.sort();

    println!("{:?} ({})", connections, connections.len());

    Ok("".to_string())
}

fn map_downstream(input: PathBuf, target_wire: &str, depth: usize) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;
    let device = puzzle_input.into_device();

    let connections = device.find_downstream(target_wire, depth);
    let mut connections: Vec<_> = connections.iter().collect();
    connections.sort();

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