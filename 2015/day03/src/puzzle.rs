use std::collections::HashSet;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub struct PuzzleInput {
    directions: Vec<Direction>
}

impl PuzzleInput {
    pub fn new(directions: Vec<Direction>) -> PuzzleInput {
        PuzzleInput { directions }
    }

    pub fn houses_visited(&self) -> usize {
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        let mut visited = HashSet::new();
        visited.insert((x, y));

        for direction in self.directions.iter() {
            match direction {
                Direction::North => y -= 1,
                Direction::East => x += 1,
                Direction::South => y += 1,
                Direction::West => x -= 1,
            }

            visited.insert((x, y));
        }
        
        visited.len()
    }
}

impl FromStr for PuzzleInput {
    type Err = ParsePuzzleInputError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut directions = vec![];

        for c in s.chars() {
            let direction = match c {
                '^' => Direction::North,
                '>' => Direction::East,
                'v' => Direction::South,
                '<' => Direction::West,
                _ => return Err(ParsePuzzleInputError::InvalidDirection(c))
            };

            directions.push(direction);
        }

        Ok(PuzzleInput::new(directions))
    }
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Error, Debug)]
pub enum ParsePuzzleInputError {
    #[error("invalid direction '{0}'")]
    InvalidDirection(char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_examples() {
        assert!(">".parse::<PuzzleInput>().unwrap().houses_visited() == 2);
        assert!("^>v<".parse::<PuzzleInput>().unwrap().houses_visited() == 4);
        assert!("^v^v^v^v^v".parse::<PuzzleInput>().unwrap().houses_visited() == 2);
    }
}
