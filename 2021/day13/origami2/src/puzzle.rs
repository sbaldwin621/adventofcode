use crate::paper::Point;

#[derive(Debug, PartialEq, Eq)]
pub struct PuzzleInput {
    pub dots: Vec<Point>,
    pub instructions: Vec<Instruction>
}

impl PuzzleInput {
    pub fn new(dots: Vec<Point>, instructions: Vec<Instruction>) -> PuzzleInput {
        PuzzleInput { dots, instructions }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    FoldUp(i64),
    FoldLeft(i64)
}