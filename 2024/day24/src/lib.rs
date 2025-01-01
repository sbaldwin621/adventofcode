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
    let mut unsolved_device = puzzle_input.into_device();

    let mut original_device = unsolved_device.clone();
    let actual_output = original_device.solve();

    let expected_output = unsolved_device.expected_output();

    let difference = expected_output.difference(&actual_output);
    let correct_wires: HashSet<String> = difference.falses().iter().cloned().collect();
    let wrong_wires: HashSet<String> = difference.trues().iter().cloned().collect();
    
    let mut interesting_true_wires = vec![];
    let mut interesting_false_wires = vec![];
    for wire in unsolved_device.wires() {
        if wire.starts_with("x") || wire.starts_with("y") {
            continue;
        }

        let value = original_device.get_value(wire).unwrap();

        let mut device = unsolved_device.clone();
        device.values_mut().insert(wire.clone(), !value);

        let tweaked_output = device.solve();
        let tweaked_difference = expected_output.difference(&tweaked_output);

        if correct_wires.is_disjoint(&tweaked_difference.trues()) {
            let score = tweaked_difference.trues().len();
            // println!("wire '{}' does not affect correct wires; {} wrong outputs", wire, score);

            if value {
                interesting_true_wires.push((wire, score));
            } else {
                interesting_false_wires.push((wire, score));
            }
        }
    }

    interesting_true_wires.sort_by_key(|(_, score)| *score);
    interesting_false_wires.sort_by_key(|(_, score)| *score);

    let mut device = unsolved_device.clone();
    device.values_mut().insert(interesting_true_wires[0].0.clone(), false);
    device.values_mut().insert(interesting_true_wires[1].0.clone(), false);
    device.values_mut().insert(interesting_true_wires[2].0.clone(), false);
    device.values_mut().insert(interesting_true_wires[3].0.clone(), false);

    device.values_mut().insert(interesting_false_wires[0].0.clone(), true);
    device.values_mut().insert(interesting_false_wires[1].0.clone(), true);
    device.values_mut().insert(interesting_false_wires[2].0.clone(), true);
    device.values_mut().insert(interesting_false_wires[3].0.clone(), true);
    
    let tweaked_output = device.solve();
    let tweaked_difference = expected_output.difference(&tweaked_output);

    println!("{}", tweaked_difference.trues().len());
    
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

    println!("{:?} ({})", connections, connections.len());

    Ok("".to_string())
}

fn map_downstream(input: PathBuf, target_wire: &str, depth: usize) -> Result<String, ApplicationError> {
    let puzzle_input = read_puzzle_input(input)?;
    let device = puzzle_input.into_device();

    let connections = device.find_downstream(target_wire, depth);

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