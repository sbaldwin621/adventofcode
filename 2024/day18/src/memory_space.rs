use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

pub struct MemorySpace {
    size: usize,
    tiles: HashMap<Position, Tile>
}

impl MemorySpace {
    pub fn solve(&self) -> Option<usize> {
        let goal: i32 = (self.size - 1).try_into().unwrap();
        let goal = &Position(goal, goal);

        let mut best_scores = HashMap::new();
        let mut walkers = vec![MemorySpaceWalker::default()];
        
        while walkers.len() > 0 {
            let mut next_walkers = vec![];

            for walker in walkers {
                if walker.pos() != goal {
                    for direction in DIRECTIONS {
                        let next_walker = walker.move_one(direction);
                        
                        if let Tile::Floor = self.tile_at(next_walker.pos()) {
                            let best_score_at_next_pos = *best_scores.get(next_walker.pos()).unwrap_or(&usize::MAX);
                            if next_walker.score() < best_score_at_next_pos {
                                best_scores.insert(*next_walker.pos(), next_walker.score());
        
                                next_walkers.push(next_walker);
                            }
                        }
                    }
                }                
            }

            walkers = next_walkers;
        }

        best_scores.get(goal).cloned()
    }

    fn tile_at(&self, position: &Position) -> Tile {
        let size: i32 = self.size.try_into().unwrap();

        let x = position.x();
        let y = position.y();

        if x < 0 || x >= size || y < 0 || y >= size {
            Tile::Wall
        } else {
            self.tiles.get(position).cloned().unwrap_or(Tile::Floor)
        }
    }
}

#[derive(Debug, Default, Clone)]
struct MemorySpaceWalker {
    pos: Position,
    score: usize
}

impl MemorySpaceWalker {
    pub fn new(&self, pos: Position) -> MemorySpaceWalker {
        let score = 0;

        MemorySpaceWalker { pos, score }
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn move_one(&self, direction: &Direction) -> MemorySpaceWalker {
        MemorySpaceWalker {
            pos: self.pos.move_one(direction),
            score: self.score + 1
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Floor,
    Wall
}

pub struct PuzzleInput {
    bytes: Vec<Position>
}

impl PuzzleInput {
    pub fn to_memory_space(&self, memory_space_size: usize, num_bytes: usize) -> MemorySpace {
        let mut tiles = HashMap::new();

        for byte in self.bytes.iter().take(num_bytes) {
            tiles.insert(*byte, Tile::Wall);
        }

        let size = memory_space_size;
        
        MemorySpace { size, tiles }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn get(&self, i: usize) -> Option<&Position> {
        self.bytes.get(i)
    }
}

impl FromStr for PuzzleInput {
    type Err = ParsePositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = vec![];

        for line in s.lines() {
            let position: Position = line.parse()?;
            bytes.push(position);
        }

        Ok(PuzzleInput { bytes })
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Position(i32, i32);

impl Position {
    #[inline]
    pub fn x(&self) -> i32 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn move_one(&self, direction: &Direction) -> Position {
        let x = self.x();
        let y = self.y();

        match direction {
            Direction::North => Position(x, y - 1),
            Direction::East => Position(x + 1, y),
            Direction::South => Position(x, y + 1),
            Direction::West => Position(x - 1, y)
        }
    }
}

impl FromStr for Position {
    type Err = ParsePositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = s.split_once(',') {
            let x: i32 = x.parse()?;
            let y: i32 = y.parse()?;

            Ok(Position(x, y))
        } else {
            Err(ParsePositionError::InvalidSyntax)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West
}

static DIRECTIONS: &[Direction] = &[Direction::North, Direction::East, Direction::South, Direction::West];

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Error, Debug)]
pub enum ParsePositionError {
    #[error("invalid syntax")]
    InvalidSyntax,
    #[error("couldn't parse int: {0}")]
    ParseIntError(#[from]ParseIntError)
}