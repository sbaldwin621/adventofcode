use std::collections::{HashMap, HashSet};
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

pub struct MazeSimulation<'a> {
    maze: &'a Maze
}

impl<'a> MazeSimulation<'a> {
    pub fn new(maze: &'a Maze) -> Self {
        MazeSimulation { maze }
    }

    pub fn simulate(&mut self, threshold: u32, cheat_length: u32) -> Option<usize> {
        let no_cheat_scores = self.simulate_no_cheating();
        let mut cheats = HashMap::new();

        for (pos, score) in no_cheat_scores.iter() {
            let mut visited = HashSet::new();
            let mut walkers = vec![MazeWalker::new(*pos, *score, cheat_length)];

            while walkers.len() > 0 {
                let mut next_walkers = vec![];

                for walker in walkers {
                    let walker_pos = walker.pos();
                    for &direction in DIRECTIONS {
                        let next_pos = walker_pos.move_one(direction);
                        if !visited.contains(&next_pos) {
                            if let Some(next_walker) = walker.with_move(next_pos) {
                                if let Tile::Floor = self.maze.tile_at(next_pos) {
                                    if let Some(next_pos_score) = no_cheat_scores.get(&next_pos) {
                                        let next_score = next_walker.score();
                                        if next_score < *next_pos_score {
                                            let savings = next_pos_score - next_score;
                                            cheats.insert((*pos, next_pos), savings);
                                        }
                                    }
                                }
                                    
                                visited.insert(next_pos);
                                next_walkers.push(next_walker);
                            }
                        }
                    }
                }

                walkers = next_walkers;
            }
        }
        
        let mut result = 0;

        let mut cheats_by_time_saved = HashMap::new();
        for (_, time_saved) in cheats {
            *cheats_by_time_saved.entry(time_saved).or_insert(0) += 1;
        }

        let mut cheats_by_time_saved: Vec<_> = cheats_by_time_saved.iter().collect();
        cheats_by_time_saved.sort_by_key(|c| c.0);

        for (time_saved, count) in cheats_by_time_saved {
            println!("{} cheats that save {} picosecond(s)", count, time_saved);

            if *time_saved >= threshold {
                result += count;
            }
        }

        Some(result)
    }

    fn simulate_no_cheating(&mut self) -> HashMap<Position, u32> {
        let mut completed_walkers = vec![];
        let mut walkers = vec![MazeWalker::new(self.maze.start_pos, 0, u32::MAX)];
        
        let mut best_scores = HashMap::new();
        best_scores.insert(self.maze.start_pos, 0);

        while walkers.len() > 0 {
            let mut next_walkers = vec![];           

            for walker in walkers {
                let pos = walker.pos();
                let score = walker.score();

                if pos == self.maze.end_pos {
                    completed_walkers.push(walker);
                    continue;
                }
                
                for &direction in DIRECTIONS {
                    let next_pos = pos.move_one(direction);
                    let next_score = score + 1;

                    if !walker.has_visited(&next_pos) {
                        if let Tile::Floor = self.maze.tile_at(next_pos) {
                            let best_score_at_next_pos = best_scores.get(&next_pos)
                                .cloned().unwrap_or(u32::MAX);
                            
                            if next_score < best_score_at_next_pos {
                                best_scores.insert( next_pos, next_score);
                                next_walkers.push(walker.with_move(next_pos).unwrap());
                            }
                        }
                    }
                }
            }

            walkers = next_walkers;
        }

        best_scores
    }
}

#[derive(Debug)]
struct MazeWalker {
    pos: Position,
    fuel: u32,
    score: u32,
    visited: HashSet<Position>
}

impl MazeWalker {
    pub fn new(pos: Position, score: u32, fuel: u32) -> MazeWalker {
        let mut visited = HashSet::new();
        visited.insert(pos);

        MazeWalker { pos, fuel, score, visited }
    }

    pub fn with_move(&self, pos: Position) -> Option<MazeWalker> {
        if self.fuel > 0 {
            let mut visited = self.visited.clone();
            visited.insert(pos);
    
            let score = self.score + 1;
            let fuel = self.fuel - 1;
            
            Some(MazeWalker { pos, score, fuel, visited })
        } else {
            None
        }        
    }

    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn has_visited(&self, pos: &Position) -> bool {
        self.visited.contains(pos)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Wall,
    Floor
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West
}

static DIRECTIONS: &[Direction] = &[Direction::North, Direction::East, Direction::South, Direction::West];

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