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
    let program: ShopProgram = contents.parse().unwrap();
    
    match options.part {
        1 => run_part1(program),
        2 => run_part2(program),
        _ => Err(ApplicationError::UnknownPart)
    }
}

fn run_part1(program: ShopProgram) -> Result<i32, ApplicationError> {
    Ok(program.evaluate(ShopLanguageVersion::One))
}

fn run_part2(program: ShopProgram) -> Result<i32, ApplicationError> {
    Ok(program.evaluate(ShopLanguageVersion::Two))
}

#[derive(Debug)]
pub struct ShopProgram {
    instructions: Vec<ShopProgramInstruction>
}

impl ShopProgram {
    pub fn new(instructions: Vec<ShopProgramInstruction>) -> ShopProgram {
        ShopProgram { instructions }
    }

    pub fn evaluate(&self, language_version: ShopLanguageVersion) -> i32 {
        match language_version {
            ShopLanguageVersion::One => self.evaluate_version_one(),
            ShopLanguageVersion::Two => self.evaluate_version_two()
        }
    }

    fn evaluate_version_one(&self) -> i32 {
        let mut accum = 0;
        
        for instruction in self.instructions.iter() {
            match instruction {
                ShopProgramInstruction::Mul(a, b) => accum += a * b,
                _ => ()
            };
        }

        accum
    }

    fn evaluate_version_two(&self) -> i32 {
        let mut accum = 0;
        let mut mul_enabled = true;
        
        for instruction in self.instructions.iter() {
            match instruction {
                ShopProgramInstruction::Mul(a, b) if mul_enabled => accum += a * b,
                ShopProgramInstruction::Do => mul_enabled = true,
                ShopProgramInstruction::Dont => mul_enabled = false,
                _ => ()
            };
        }

        accum
    }
}

impl FromStr for ShopProgram {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = vec![];
        
        let re = Regex::new(r"(mul|don't|do)\((?:(\d{1,3}),(\d{1,3}))?\)").unwrap();
        
        for captures in re.captures_iter(s) {
            let operation = &captures[1];
            let shop_program_instruction = match operation {
                "mul" if captures.len() == 4 => {
                    let a: i32 = captures[2].parse().unwrap();
                    let b: i32 = captures[3].parse().unwrap();

                    Some(ShopProgramInstruction::Mul(a, b))
                },
                "do" => Some(ShopProgramInstruction::Do),
                "don't" => Some(ShopProgramInstruction::Dont),
                _ => None
            };

            if let Some(instruction) = shop_program_instruction {
                instructions.push(instruction);
            }
        }

        Ok(ShopProgram::new(instructions))
    }
}

#[derive(Debug)]
pub enum ShopLanguageVersion {
    One,
    Two
}

#[derive(Debug)]
pub enum ShopProgramInstruction {
    Mul(i32, i32),
    Do,
    Dont
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] std::io::Error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn shop_language_version_one() {
        let program: ShopProgram = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".parse().unwrap();
        assert_eq!(program.evaluate(ShopLanguageVersion::One), 161);
    }

    #[test]
    pub fn shop_language_version_two() {
        let program: ShopProgram = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".parse().unwrap();
        assert_eq!(program.evaluate(ShopLanguageVersion::Two), 48);
    }
}