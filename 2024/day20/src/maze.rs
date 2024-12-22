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
    maze: &'a Maze,
    best_scores: HashMap<Position, u32>
}

impl<'a> MazeSimulation<'a> {
    pub fn new(maze: &'a Maze) -> Self {
        let best_scores = HashMap::new();

        MazeSimulation { maze, best_scores }
    }

    pub fn simulate(&mut self) -> Option<MazeSolution> {
        let mut completed_walkers = vec![];
        let mut walkers = vec![MazeWalker::new(self.maze.start_pos)];
        
        let mut best_score = u32::MAX;

        while walkers.len() > 0 {
            let mut next_walkers = vec![];           

            for walker in walkers {
                let pos = walker.pos();
                let score = walker.score();

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
                        let best_score_at_next_pos = self.best_scores.get(&next_pos)
                            .cloned().unwrap_or(u32::MAX);

                        if next_score < best_score_at_next_pos {
                            self.best_scores.insert(next_pos, next_score);
                            next_walkers.push(walker.with_move(next_pos));
                        }
                    }
                }
            }

            walkers = next_walkers;
        }

        if let Some(best_score) = self.best_scores.get(&self.maze.end_pos).cloned() {
            let mut all_best_path_tiles = HashSet::new();
            
            for walker in completed_walkers {
                if walker.score() == best_score {
                    for pos in walker.path().iter() {
                        all_best_path_tiles.insert(*pos);
                    }
                }
            }
        
            let best_path_tile_count = all_best_path_tiles.len();
            
            Some(MazeSolution { best_score, best_path_tile_count })
        } else {
            None
        }
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
    has_cheated: bool,
    path: Vec<Position>
}

impl MazeWalker {
    pub fn new(pos: Position) -> MazeWalker {
        let score = 0;
        let has_cheated = false;
        let path = vec![];

        MazeWalker { pos, score, has_cheated, path }
    }

    pub fn with_move(&self, pos: Position) -> MazeWalker {
        let mut path = self.path.clone();
        path.push(pos);

        let score = self.score + 1;
        let has_cheated = self.has_cheated;

        MazeWalker { pos, score, has_cheated, path }
    }

    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn has_cheated(&self) -> bool {
        self.has_cheated
    }

    pub fn path(&self) -> &Vec<Position> {
        &self.path
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