use std::collections::HashSet;
use std::panic;
use std::str::FromStr;

use thiserror::Error;

#[derive(Clone, Copy)]
pub enum LabMapTile {
    Empty,
    Obstacle,
    GuardStartingPosition,
    MapBoundary
}

pub struct LabMap {
    rows: Vec<LabMapRow>
}

impl LabMap {
    pub fn new(rows: Vec<LabMapRow>) -> LabMap {
        LabMap { rows }
    }

    pub fn guard_starting_position(&self) -> Option<(i32, i32)> {
        for (y, row) in self.rows.iter().enumerate() {
            for (x, tile) in row.tiles.iter().enumerate() {
                if let LabMapTile::GuardStartingPosition = tile {
                    let x: i32 = x.try_into().unwrap();
                    let y: i32 = y.try_into().unwrap();

                    return Some((x, y));
                }
            }
        }

        None
    }

    pub fn tile_at(&self, x: i32, y: i32) -> LabMapTile {
        if x < 0 || y < 0 {
            LabMapTile::MapBoundary
        } else {
            let x: usize = x.try_into().unwrap();
            let y: usize = y.try_into().unwrap();

            if let Some(row) = self.rows.get(y) {
                if let Some(tile) = row.tiles.get(x) {
                    *tile
                } else {
                    LabMapTile::MapBoundary
                }
            } else {
                LabMapTile::MapBoundary
            }
        }
    }
}

pub struct LabMapRow {
    tiles: Vec<LabMapTile>
}

impl LabMapRow {
    pub fn new(tiles: Vec<LabMapTile>) -> LabMapRow {
        LabMapRow { tiles }
    }
}

impl FromStr for LabMapRow {
    type Err = LabMapRowParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = vec![];

        for char in s.chars() {
            let tile = match char {
                '.' => LabMapTile::Empty,
                '#' => LabMapTile::Obstacle,
                '^' => LabMapTile::GuardStartingPosition,
                _ => return Err(LabMapRowParseError::UnexpectedCharacter(char))
            };
            
            tiles.push(tile);
        }
        
        Ok(LabMapRow::new(tiles))
    }    
}

#[derive(Debug, Error)]
pub enum LabMapRowParseError {
    #[error("unexpected character '{0}'")]
    UnexpectedCharacter(char)
}

pub struct GuardSimulation<'a> {
    map: &'a LabMap,
    x: i32,
    y: i32,
    heading: Direction,
    locations_visited: HashSet<(i32, i32)>
}

impl<'a> GuardSimulation<'a> {
    pub fn new(map: &'a LabMap) -> GuardSimulation<'a> {
        if let Some((x, y)) = map.guard_starting_position() {
            let heading = Direction::North;
            
            let mut locations_visited = HashSet::new();
            locations_visited.insert((x, y));
            
            GuardSimulation { map, x, y, heading, locations_visited }
        } else {
            panic!("no guard starting position");
        }
    }

    pub fn step(&mut self) -> bool {
        let (x_delta, y_delta) = match self.heading {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0)
        };

        let next_x = self.x + x_delta;
        let next_y = self.y + y_delta;
        let next_tile = self.map.tile_at(next_x, next_y);

        match next_tile {
            LabMapTile::Empty | LabMapTile::GuardStartingPosition => {
                self.x = next_x;
                self.y = next_y;

                self.locations_visited.insert((self.x, self.y));
                
                return true;
            },
            LabMapTile::Obstacle => {
                self.heading = self.heading.rotate_right();

                return true;
            },
            LabMapTile::MapBoundary => {
                return false;
            }
        }
    }

    pub fn visited_count(&self) -> usize {
        self.locations_visited.len()
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }
}