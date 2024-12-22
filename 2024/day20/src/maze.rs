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

    pub fn simulate(&mut self, threshold: u32) -> Option<MazeSolution> {
        let mut completed_walkers = vec![];
        let mut walkers = vec![MazeWalker::new(self.maze.start_pos)];
        
        let no_cheat_score = match self.simulate_no_cheating() {
            Some(score) => score,
            None => return None
        };
        
        let mut best_scores = HashMap::new();
        let mut best_score = u32::MAX;

        while walkers.len() > 0 {
            let mut next_walkers = vec![];           

            for walker in walkers {
                let pos = walker.pos();
                let score = walker.score();
                let has_cheated = walker.has_cheated();

                if pos == self.maze.end_pos {
                    if score < best_score {
                        best_score = score;
                    }

                    completed_walkers.push(walker);
                    continue;
                }

                for &direction in DIRECTIONS {
                    let next_pos = pos.move_one(direction);
                    let next_score = score + 1;
                    
                    if let Tile::Floor = self.maze.tile_at(next_pos) {
                        if !walker.has_visited(&next_pos) {
                            let best_score_at_next_pos = best_scores.get(&(has_cheated, next_pos))
                                .cloned().unwrap_or(u32::MAX);

                            let should_continue = if next_score < best_score_at_next_pos {
                                best_scores.insert((has_cheated, next_pos), next_score);
                                true
                            } else if next_score < no_cheat_score - threshold {
                                true
                            } else {
                                false
                            };

                            if should_continue {
                                next_walkers.push(walker.with_move(next_pos));
                            }
                        }
                    } else if !has_cheated {
                        let cheat_pos = next_pos.move_one(direction);
                        let next_score = next_score + 1;

                        if !walker.has_visited(&cheat_pos) {
                            if let Tile::Floor = self.maze.tile_at(cheat_pos) {
                                let best_score_at_cheat_pos = best_scores.get(&(true, cheat_pos))
                                    .cloned().unwrap_or(u32::MAX);
    
                                let should_continue = if next_score < best_score_at_cheat_pos {
                                    best_scores.insert((true, cheat_pos), next_score);
                                    true
                                } else if next_score < no_cheat_score - threshold {
                                    true
                                } else {
                                    false
                                };
    
                                if should_continue {
                                    next_walkers.push(walker.with_cheat((next_pos, cheat_pos)));
                                }
                            }
                        }
                    }
                }
            }

            walkers = next_walkers;
        }

        let mut cheats = HashMap::new();
        for walker in completed_walkers {
            if walker.has_cheated() {
                cheats.entry(walker.score()).and_modify(|c| *c += 1).or_insert(1_usize);
            }
        }
        
        let mut cheats: Vec<_> = cheats.iter().collect();
        cheats.sort_by(|a, b| b.0.cmp(a.0));

        for (score, count) in cheats {
            let time_saved = no_cheat_score - score;

            println!("{} cheats that save {} picosecond(s)", count, time_saved);
        }

        if let Some(best_score) = best_scores.get(&(true, self.maze.end_pos)).cloned() {
            
            
            let best_path_tile_count = 0;
            
            Some(MazeSolution { best_score, best_path_tile_count })
        } else {
            None
        }
    }

    fn simulate_no_cheating(&mut self) -> Option<u32> {
        let mut completed_walkers = vec![];
        let mut walkers = vec![MazeWalker::new(self.maze.start_pos)];
        
        let mut best_scores = HashMap::new();

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
                                next_walkers.push(walker.with_move(next_pos));
                            }
                        }
                    }                    
                }
            }

            walkers = next_walkers;
        }

        best_scores.get(&self.maze.end_pos).cloned()
    }

    fn print(&self, all_best_paths: &HashSet<Position>) {
        for y in 0..self.maze.height {
            for x in 0..self.maze.width {
                let pos = Position(x, y);
                let char =
                    if all_best_paths.contains(&pos) {
                        'O'
                    } else if pos == self.maze.start_pos {
                        'S'
                    } else if pos == self.maze.end_pos {
                        'E'
                    } else {
                        match self.maze.tile_at(Position(x, y)) {
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

#[derive(Debug)]
struct MazeWalker {
    pos: Position,
    score: u32,
    cheat: Option<(Position, Position)>,
    visited: HashSet<Position>
}

impl MazeWalker {
    pub fn new(pos: Position) -> MazeWalker {
        let score = 0;
        let cheat = None;
        let visited = HashSet::new();

        MazeWalker { pos, score, cheat, visited }
    }

    pub fn with_move(&self, pos: Position) -> MazeWalker {
        let mut visited = self.visited.clone();
        visited.insert(pos);

        let score = self.score + 1;
        let cheat = self.cheat;

        MazeWalker { pos, score, cheat, visited }
    }

    pub fn with_cheat(&self, cheat: (Position, Position)) -> MazeWalker {
        let pos = cheat.1;
        
        let mut visited = self.visited.clone();
        visited.insert(cheat.0);
        visited.insert(cheat.1);

        let score = self.score + 2;
        let cheat = Some(cheat);
        
        MazeWalker { pos, score, cheat, visited }
    }

    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn cheat(&self) -> &Option<(Position, Position)> {
        &self.cheat
    }

    pub fn has_cheated(&self) -> bool {
        self.cheat.is_some()
    }

    pub fn has_visited(&self, pos: &Position) -> bool {
        self.visited.contains(pos)
    }
}

#[derive(Debug)]
pub struct MazeSolution {
    best_score: u32,
    best_path_tile_count: usize
}

impl MazeSolution {
    pub fn best_score(&self) -> u32 {
        self.best_score
    }

    pub fn best_path_tile_count(&self) -> usize {
        self.best_path_tile_count
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

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East
        }
    }

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