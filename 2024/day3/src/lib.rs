use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::str::FromStr;

use clap::Parser;
use regex::Regex;
use thiserror::Error;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<i32, ApplicationError> {
    let filename = options.filename;

    let contents = std::fs::read_to_string(filename)?;
    
    match options.part {
        1 => run_part1(contents),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }
}

fn run_part1(contents: String) -> Result<i32, ApplicationError> {
    let program: ShopProgram = contents.parse().unwrap();
    Ok(program.evaluate())
}

fn run_part2() -> Result<i32, ApplicationError> {
    !unimplemented!()
}

#[derive(Debug)]
pub struct ShopProgram {
    instructions: Vec<ShopProgramInstruction>
}

impl ShopProgram {
    pub fn new(instructions: Vec<ShopProgramInstruction>) -> ShopProgram {
        ShopProgram { instructions }
    }

    pub fn evaluate(&self) -> i32 {
        self.instructions.iter()
            .fold(0, |accum, instruction| accum + instruction.evaluate())
    }
}

impl FromStr for ShopProgram {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = vec![];
        
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        for (_, [a, b]) in re.captures_iter(s).map(|c| c.extract()) {
            let a: i32 = a.parse().unwrap();
            let b: i32 = b.parse().unwrap();

            instructions.push(ShopProgramInstruction::Mul(a, b));
        }

        Ok(ShopProgram::new(instructions))
    }
}

#[derive(Debug)]
pub enum ShopProgramInstruction {
    Mul(i32, i32)
}

impl ShopProgramInstruction {
    pub fn evaluate(&self) -> i32 {
        match self {
            ShopProgramInstruction::Mul(a, b) => a * b,
        }
    }
}


#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let program: ShopProgram = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".parse().unwrap();
        assert_eq!(program.evaluate(), 161);
    }
}