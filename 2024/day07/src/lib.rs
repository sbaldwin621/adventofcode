use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use calibration::{UnsolvedCalibration, UnsolvedCalibrationParseError, UnsolvedCalibrationSet};
use clap::Parser;
use thiserror::Error;

mod calibration;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<u64, ApplicationError> {
    let filename = options.filename;

    let mut calibrations = vec![];

    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;
        let calibration: UnsolvedCalibration = line.parse()?;

        calibrations.push(calibration);
    }

    let calibration_set = UnsolvedCalibrationSet::new(calibrations);

    match options.part {
        1 => run_part1(&calibration_set),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }
}

fn run_part1(calibration_set: &UnsolvedCalibrationSet) -> Result<u64, ApplicationError> {
    !unimplemented!()
}

fn run_part2() -> Result<u64, ApplicationError> {
    !unimplemented!()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("couldn't parse puzzle input line: {0}")]
    CouldntParseLine(#[from] UnsolvedCalibrationParseError)
}