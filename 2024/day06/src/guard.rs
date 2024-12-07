use std::collections::{hash_set, HashSet};
use std::hash::Hash;
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

    pub fn width(&self) -> usize {
        self.rows[0].tiles.len()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn iter(&self) -> LabMapIterator<'_> {
        let next = Some((0, 0, self.tile_at(0, 0)));

        LabMapIterator { map: self, current: None, next }
    }
}

pub struct LabMapIterator<'a> {
    map: &'a LabMap,
    current: Option<(i32, i32, LabMapTile)>,
    next: Option<(i32, i32, LabMapTile)>
}

impl<'a> Iterator for LabMapIterator<'a> {
    type Item = (i32, i32, LabMapTile);

    fn next(&mut self) -> Option<Self::Item> {
        self.current = self.next;

        if let Some((x, y, _)) = self.current {
            let mut x = x + 1;
            let mut y = y;
            
            if x >= self.map.width().try_into().unwrap() {
                x = 0;
                y += 1;
            }
    
            if y >= self.map.height().try_into().unwrap() {
                self.next = None;
            } else {
                let tile = self.map.tile_at(x, y);

                self.next = Some((x, y, tile));
            }
        }

        self.current
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
    extra_obstacle: Option<(i32, i32)>,
    x: i32,
    y: i32,
    heading: Direction,
    locations_visited: HashSet<(i32, i32)>,
    locations_and_headings_visited: HashSet<(i32, i32, Direction)>,
    loop_detected: bool
}

impl<'a> GuardSimulation<'a> {
    pub fn new(map: &'a LabMap, extra_obstacle: Option<(i32, i32)>) -> GuardSimulation<'a> {
        if let Some((x, y)) = map.guard_starting_position() {
            let heading = Direction::North;
            
            let mut locations_visited = HashSet::new();
            locations_visited.insert((x, y));

            let mut locations_and_headings_visited = HashSet::new();
            locations_and_headings_visited.insert((x, y, heading));
            
            GuardSimulation { map, extra_obstacle, x, y, heading, locations_visited, locations_and_headings_visited, loop_detected: false }
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

        let next_tile = self.tile_at(next_x, next_y);

        match next_tile {
            LabMapTile::Empty | LabMapTile::GuardStartingPosition => {
                self.x = next_x;
                self.y = next_y;

                self.locations_visited.insert((self.x, self.y));
                
                if !self.locations_and_headings_visited.insert((self.x, self.y, self.heading)) {
                    self.loop_detected = true;
                }
                
                return true;
            },
            LabMapTile::Obstacle => {
                self.heading = self.heading.rotate_right();

                if !self.locations_and_headings_visited.insert((self.x, self.y, self.heading)) {
                    self.loop_detected = true;
                }
                
                return true;
            },
            LabMapTile::MapBoundary => {
                return false;
            }
        }
    }

    fn tile_at(&self, x: i32, y: i32) -> LabMapTile {
        if let Some((obstacle_x, obstacle_y)) = self.extra_obstacle {
            if obstacle_x == x && obstacle_y == y {
                return LabMapTile::Obstacle;
            }
        }

        self.map.tile_at(x, y)
    }

    pub fn loop_detected(&self) -> bool {
        self.loop_detected
    }

    pub fn visited_count(&self) -> usize {
        self.locations_visited.len()
    }

    pub fn locations_visited(&self) -> hash_set::Iter<'_, (i32, i32)> {
        self.locations_visited.iter()
    }
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West
}

impl Eq for Direction {
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