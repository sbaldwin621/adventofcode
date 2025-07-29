use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

use regex::Regex;
use thiserror::Error;

#[derive(Debug)]
pub struct PuzzleInput {
    instructions: Vec<Instruction>
}

impl PuzzleInput {
    pub fn new(instructions: Vec<Instruction>) -> PuzzleInput {
        PuzzleInput { instructions }
    }

    pub fn evaluate_part1(&self) -> usize {
        let mut light_grid = LightGrid::new();

        for instruction in self.instructions.iter() {
            light_grid.apply_instruction_part1(instruction);
        }

        light_grid.total_brightness()
    }

    pub fn evaluate_part2(&self) -> usize {
        let mut light_grid = LightGrid::new();

        for instruction in self.instructions.iter() {
            light_grid.apply_instruction_part2(instruction);
        }

        light_grid.total_brightness()
    }
}

impl FromStr for PuzzleInput {
    type Err = ParsePuzzleInputError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?m)^(turn on|turn off|toggle) (\d{1,3}),(\d{1,3}) through (\d+),(\d+)$").unwrap();
    
        let mut instructions = Vec::new();

        for (_, [operation, x1, y1, x2, y2]) in re.captures_iter(&s).map(|c| c.extract()) {
            let operation = match operation {
                "turn on" => Operation::TurnOn,
                "turn off" => Operation::TurnOff,
                "toggle" => Operation::Toggle,
                _ => return Err(ParsePuzzleInputError::UnknownOperation(operation.to_owned()))
            };

            let x1 = x1.parse::<usize>()?;
            let y1 = y1.parse::<usize>()?;
            let p1 = Point(x1, y1);

            let x2 = x2.parse::<usize>()?;
            let y2 = y2.parse::<usize>()?;
            let p2 = Point(x2, y2);

            let instruction = Instruction::new(operation, p1, p2);

            instructions.push(instruction);
        }

        Ok(PuzzleInput::new(instructions))
    }
}

#[derive(Error, Debug)]
pub enum ParsePuzzleInputError {
    #[error("unknown operation {0}")]
    UnknownOperation(String),
    #[error("couldn't parse integer in instruction: {0}")]
    ParseIntError(#[from]ParseIntError)
}

#[derive(Debug)]
pub enum Operation {
    TurnOn,
    TurnOff,
    Toggle
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point(usize, usize);

#[derive(Debug)]
pub struct Instruction {
    operation: Operation,
    point1: Point,
    point2: Point
}

impl Instruction {
    pub fn new(operation: Operation, point1: Point, point2: Point) -> Instruction {
        Instruction { operation, point1, point2 }
    }
}

#[derive(Debug)]
pub struct LightGrid {
    lights: HashMap<Point, usize>
}

impl LightGrid {
    pub fn new() -> LightGrid {
        LightGrid { lights: HashMap::new() }
    }

    pub fn apply_instruction_part1(&mut self, instruction: &Instruction) {
        for y in instruction.point1.1..=instruction.point2.1 {
            for x in instruction.point1.0..=instruction.point2.0 {
                let point = Point(x, y);
                match instruction.operation {    
                    Operation::TurnOn => {
                        self.lights.insert(point, 1);
                    }
                    Operation::TurnOff => {
                        self.lights.remove(&point);
                    }
                    Operation::Toggle => {
                        if self.lights.contains_key(&point) {
                            self.lights.remove(&point);
                        } else {
                            self.lights.insert(point, 1);
                        }
                    }
                };
            }
        }
    }

    pub fn apply_instruction_part2(&mut self, instruction: &Instruction) {
        for y in instruction.point1.1..=instruction.point2.1 {
            for x in instruction.point1.0..=instruction.point2.0 {
                let point = Point(x, y);
                match instruction.operation {    
                    Operation::TurnOn => {
                        if let Some(brightness) = self.lights.get_mut(&point) {
                            *brightness += 1;
                        } else {
                            self.lights.insert(point, 1);
                        }
                    }
                    Operation::TurnOff => {
                        if let Some(brightness) = self.lights.get_mut(&point) {
                            if *brightness > 1 {
                                *brightness -= 1;
                            } else {
                                self.lights.remove(&point);
                            }
                        }
                    }
                    Operation::Toggle => {
                        if let Some(brightness) = self.lights.get_mut(&point) {
                            *brightness += 2;
                        } else {
                            self.lights.insert(point, 2);
                        }
                    }
                };
            }
        }
    }

    pub fn total_brightness(&self) -> usize {
        self.lights.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_examples() {
        let mut light_grid = LightGrid::new();

        light_grid.apply_instruction_part1(&Instruction::new(Operation::TurnOn, Point(0, 0), Point(999, 999)));
        assert_eq!(light_grid.total_brightness(), 1_000_000);
        
        light_grid.apply_instruction_part1(&Instruction::new(Operation::TurnOff, Point(0, 0), Point(499, 999)));
        assert_eq!(light_grid.total_brightness(), 500_000);

        light_grid.apply_instruction_part1(&Instruction::new(Operation::Toggle, Point(250, 0), Point(749, 999)));
        assert_eq!(light_grid.total_brightness(), 500_000);
    }
}
