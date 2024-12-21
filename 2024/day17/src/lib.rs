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
        2 => run_part2(&debugger_info),
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


fn run_part2(debugger_info: &DebuggerInfo) -> Result<String, ApplicationError> {
    let original_length = debugger_info.original_program().len();

    // let mut low = 0;
    // let mut high = 10usize.pow(debugger_info.original_program().len().try_into().unwrap());
    // 8.pow(program_length - 1)..8.pow(program_length)
    //  35_184_372_088_832
    // 281_474_976_710_655

    // [
    //     0o10000000_00000000,
    //     0o10000000_00000000,
    //     0o10000000_00000000,
    //     0o20000000_00000000,
    //     0o30000000_00000000,
    //     0o40000000_00000000,
    //     0o50000000_00000000,
    //     0o60000000_00000000,
    //     0o70000000_00000000,
    //     0o77777777_77777777
    // ]

    println!("{}", 0o61120000_00000000usize - 0o61110000_00000000usize);

    for n in [
        0o61110000_00000000,
        0o61110100_00000000,
        0o61110200_00000000,
        0o61110300_00000000,
        0o61110400_00000000,
        0o61110500_00000000,
        0o61110600_00000000,
        0o61110700_00000000,
        
        0o61111000_00000000,
        0o61112000_00000000,
        0o61113000_00000000,
        0o61114000_00000000,
        0o61115000_00000000,
        0o61116000_00000000,
        0o61117000_00000000
    ] {
        for guess in n.. {
            let mut emulator = Emulator::from_debugger_info(debugger_info);
            *emulator.register_a_mut() = guess;
    
            let mut a_values = vec![];
            while emulator.step() {
                a_values.push(emulator.register_a());
            }
    
            let a_string: Vec<String> = a_values.iter().map(|v| v.to_string()).collect();
            let a_string = a_string.join(",");
    
            let output = emulator.output_buffer();
            let output_string = output.iter().map(|v| v.to_string()).collect::<Vec<String>>();
            
            println!("{}: {} ({})", guess, output_string.join(","), output.len());
    
            // println!("{}: {}", guess, a_string);
    
            // if output.len() < original_length {
            //     low = guess + 1;
            //     continue;
            // } else if output.len() > original_length {
            //     high = guess - 1;
            //     continue;
            // }
    
            // let mut match_count = 0;
            // for (a, b) in debugger_info.original_program().iter().rev().zip(output.iter().rev()) {
            //     if a == b {
            //         match_count += 1;
            //     } else {
            //         break;
            //     }
            // }
    
            // if match_count == original_length {
            //     return Ok(guess.to_string());
            // }
            break;
        }
    }
    

    unreachable!()
}


fn run_part2_a(debugger_info: &DebuggerInfo) -> Result<String, ApplicationError> {
    let original_length = debugger_info.original_program().len();

    let mut low = 0;
    let mut high = 10usize.pow(debugger_info.original_program().len().try_into().unwrap());

    loop {
        let guess = (high - low) / 2;

        let mut emulator = Emulator::from_debugger_info(debugger_info);
        *emulator.register_a_mut() = guess;

        while emulator.step() { }

        let output = emulator.output_buffer();
        let output_string = output.iter().map(|v| v.to_string()).collect::<Vec<String>>();
        
        println!("{}: {}", guess, output_string.join(","));

        if output.len() < original_length {
            low = guess + 1;
            continue;
        } else if output.len() > original_length {
            high = guess - 1;
            continue;
        }

        let mut match_count = 0;
        for (a, b) in debugger_info.original_program().iter().rev().zip(output.iter().rev()) {
            if a == b {
                match_count += 1;
            } else {
                break;
            }
        }

        if match_count == original_length {
            return Ok(guess.to_string());
        }

        
    }

    unreachable!()
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