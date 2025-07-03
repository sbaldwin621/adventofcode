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

    pub fn first_basement(&self) -> Option<usize> {
        let mut current_floor = 0;
        for (i, instruction) in self.instructions.iter().enumerate() {
            match instruction {
                Instruction::Up => current_floor += 1,
                Instruction::Down => current_floor -= 1,
            }

            if current_floor == -1 {
                return Some(i + 1);
            }
        }

        None
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

    #[test]
    pub fn part2_examples() {
        assert!(")".parse::<PuzzleInput>().unwrap().first_basement() == Some(1));
        assert!("()())".parse::<PuzzleInput>().unwrap().first_basement() == Some(5));
    }
}
