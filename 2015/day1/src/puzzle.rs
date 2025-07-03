use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub struct PuzzleInput {
    instructions: Vec<Instruction>
}

impl PuzzleInput {
    pub fn new(instructions: Vec<Instruction>) -> PuzzleInput {
        PuzzleInput { instructions }
    }

    pub fn eval(&self) -> i32 {
        self.instructions.iter().map(|i| match i {
            Instruction::Up => 1,
            Instruction::Down => -1,
        }).sum()
    }
}

impl FromStr for PuzzleInput {
    type Err = ParsePuzzleInputError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = vec![];

        for c in s.chars() {
            let instruction = match c {
                '(' => Instruction::Up,
                ')' => Instruction::Down,
                _ => return Err(ParsePuzzleInputError::UnknownInstruction(c))
            };

            instructions.push(instruction);
        }

        Ok(PuzzleInput::new(instructions))
    }
}

#[derive(Debug)]
pub enum Instruction {
    Up,
    Down
}

#[derive(Error, Debug)]
pub enum ParsePuzzleInputError {
    #[error("unknown instruction: {0}")]
    UnknownInstruction(char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_examples() {
        assert!("(())".parse::<PuzzleInput>().unwrap().eval() == 0);
        assert!("()()".parse::<PuzzleInput>().unwrap().eval() == 0);

        assert!("))(((((".parse::<PuzzleInput>().unwrap().eval() == 3);
        
        assert!("())".parse::<PuzzleInput>().unwrap().eval() == -1);
        assert!("))(".parse::<PuzzleInput>().unwrap().eval() == -1);

        assert!(")))".parse::<PuzzleInput>().unwrap().eval() == -3);
        assert!(")())())".parse::<PuzzleInput>().unwrap().eval() == -3);
    }
}
