use std::collections::HashMap;
use std::str::FromStr;

use thiserror::Error;

pub struct Maze {
    start_pos: Position,
    end_pos: Position,
    width: i32,
    height: i32,
    tiles: HashMap<Position, Tile>
}

impl Maze {
    pub fn tile_at(&self, position: Position) -> Tile {
        if let Some(tile) = self.tiles.get(&position) {
            *tile
        } else {
            Tile::Wall
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position(x, y);
                let char =
                    if pos == self.start_pos {
                        'S'
                    } else if pos == self.end_pos {
                        'E'
                    } else {
                        match self.tile_at(Position(x, y)) {
                            Tile::Wall => '#',
                            Tile::Floor => '.',
                        }
                    };
                
                print!("{}", char);
            }

            println!();
        }
    }
}

impl FromStr for Maze {
    type Err = ParseMazeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_pos = None;
        let mut end_pos = None;

        let mut width = 0;
        let mut height = 0;

        let mut tiles = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            let y: i32 = y.try_into().unwrap();
            height = y + 1;

            for (x, char) in line.chars().enumerate() {
                let x: i32 = x.try_into().unwrap();
                width = x + 1;

                let pos = Position(x, y);
                let tile = match char {
                    '#' => Tile::Wall,
                    '.' => Tile::Floor,
                    'S' => {
                        start_pos = Some(pos);
                        Tile::Floor
                    },
                    'E' => {
                        end_pos = Some(pos);
                        Tile::Floor
                    },
                    _ => { 
                        return Err(ParseMazeError::UnrecognizedCharacter(char));
                    }
                };

                tiles.insert(pos, tile);
            }
        }

        if let Some(start_pos) = start_pos {
            if let Some(end_pos) = end_pos {
                Ok(Maze { start_pos, end_pos, width, height, tiles })            
            } else {
                Err(ParseMazeError::MissingEndPosition)
            }    
        } else {
            Err(ParseMazeError::MissingStartPosition)
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseMazeError {
    #[error("unrecognized character: {0}")]
    UnrecognizedCharacter(char),
    #[error("missing start position")]
    MissingStartPosition,
    #[error("missing end position")]
    MissingEndPosition
}

pub struct MazeSimulation {

}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Wall,
    Floor
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }

    pub fn rotate_counterclockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Position(i32, i32);

impl Position {
    pub fn move_one(&self, direction: Direction) -> Position {
        let x = self.0;
        let y = self.1;

        match direction {
            Direction::North => Position(x, y - 1),
            Direction::East => Position(x + 1, y),
            Direction::South => Position(x, y + 1),
            Direction::West => Position(x - 1, y)
        }
    }
}