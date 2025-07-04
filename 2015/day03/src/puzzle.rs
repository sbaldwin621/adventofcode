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
        let mut santa = Santa::new();

        let mut visited = HashSet::new();
        visited.insert(santa.pos());

        for direction in self.directions.iter() {
            santa.go(*direction);
            visited.insert(santa.pos());
        }
        
        visited.len()
    }

     pub fn houses_visited_with_robosanta(&self) -> usize {
        let mut santa = Santa::new();
        let mut robosanta = Santa::new();

        let mut visited = HashSet::new();
        visited.insert(santa.pos());

        let mut iterator = self.directions.iter();
        loop {
            if let Some(direction) = iterator.next() {
                santa.go(*direction);
                visited.insert(santa.pos());
            } else {
                break;
            }

            if let Some(direction) = iterator.next() {
                robosanta.go(*direction);
                visited.insert(robosanta.pos());
            } else {
                break;
            }
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
pub struct Santa {
    x: i32,
    y: i32
}

impl Santa {
    pub fn new() -> Santa {
        Santa { x: 0, y: 0 }
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn go(&mut self, direction: Direction) {
        match direction {
            Direction::North => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
        }        
    }
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

    #[test]
    pub fn part2_examples() {
        assert!("^v".parse::<PuzzleInput>().unwrap().houses_visited_with_robosanta() == 3);
        assert!("^>v<".parse::<PuzzleInput>().unwrap().houses_visited_with_robosanta() == 3);
        assert!("^v^v^v^v^v".parse::<PuzzleInput>().unwrap().houses_visited_with_robosanta() == 11);
    }
}
