use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::usize;

use thiserror::Error;

pub fn solve(code: &str, directional_keypad_count: usize) -> usize {
    let mut keypads = vec![
        Keypad::numeric_keypad()
    ];

    for _ in 0..directional_keypad_count {
        keypads.push(Keypad::directional_keypad());
    }

    let mut cache = HashMap::new();

    let solution = solve_segment(code, &keypads, 0, &mut cache);

    let value = code_numeric_value(code).unwrap();
    let complexity = solution * value;

    println!("{}: {}", code, solution);
    println!("{}: {} * {} = {}", code, solution, value, complexity);

    complexity
}

fn solve_segment(code: &str, keypads: &Vec<Keypad>, keypad_i: usize, cache: &mut HashMap<(usize, String), usize>) -> usize {
    let cache_key = (keypad_i, code.to_string());
    if let Some(value) = cache.get(&cache_key) {
        *value
    } else {
        let value = if let Some(keypad) = keypads.get(keypad_i) {
            let solution = keypad.solve_code(&code);
            let mut accum = 0;
    
            for segment in solution {
                let mut best_score = usize::MAX;
    
                for code in segment {
                    let score = solve_segment(&code, keypads, keypad_i + 1, cache);                
                    if score < best_score {
                        best_score = score;
                    }
                }
    
                accum += best_score;
            }            
    
            accum
        } else {
            code.len()
        };

        cache.insert(cache_key, value);

        value
    }
}

pub fn simulate(code: &str) -> Option<String> {
    let keypads = [
        Keypad::directional_keypad(),
        Keypad::directional_keypad(),
        Keypad::numeric_keypad()
    ];

    let mut current_code = code.to_string();
    for keypad in keypads {
        current_code = keypad.simulate(&current_code).unwrap();
    }

    Some(current_code)
}

fn code_numeric_value(code: &str) -> Option<usize> {
    let digits: String = code.chars().filter(|c| c.is_digit(10)).collect();
    digits.parse().ok()
}

fn path_to_code(path: &Vec<Direction>) -> String {
    let mut s = String::with_capacity(path.len() + 1);

    for direction in path {
        let char = match direction {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        };

        s.push(char);
    }

    s.push('A');

    s
}

#[derive(Debug)]
pub struct Keypad {
    pos_to_key: HashMap<Position, char>,
    key_to_pos: HashMap<char, Position>
}

impl Keypad {
    pub fn numeric_keypad() -> Keypad {
        let mut keys = HashMap::new();
        
        keys.insert(Position(0, 0), '7');
        keys.insert(Position(1, 0), '8');
        keys.insert(Position(2, 0), '9');

        keys.insert(Position(0, 1), '4');
        keys.insert(Position(1, 1), '5');
        keys.insert(Position(2, 1), '6');

        keys.insert(Position(0, 2), '1');
        keys.insert(Position(1, 2), '2');
        keys.insert(Position(2, 2), '3');

        keys.insert(Position(1, 3), '0');
        keys.insert(Position(2, 3), 'A');

        Keypad::from_positions_to_keys(keys)
    }

    pub fn directional_keypad() -> Keypad {
        let mut keys = HashMap::new();
        
        keys.insert(Position(1, 0), '^');
        keys.insert(Position(2, 0), 'A');

        keys.insert(Position(0, 1), '<');
        keys.insert(Position(1, 1), 'v');
        keys.insert(Position(2, 1), '>');

        Keypad::from_positions_to_keys(keys)
    }

    fn from_positions_to_keys(pos_to_key: HashMap<Position, char>) -> Keypad {
        let key_to_pos: HashMap<_, _> = pos_to_key
            .iter()
            .map(|(&k, &v)| (v, k))
            .collect();
        
        Keypad { pos_to_key, key_to_pos }
    }

    pub fn get_pos_for_key(&self, key: char) -> Option<Position> {
        self.key_to_pos.get(&key).cloned()
    }

    pub fn get_key_for_pos(&self, pos: &Position) -> Option<char> {
        self.pos_to_key.get(pos).cloned()
    }

    pub fn simulate(&self, code: &str) -> Option<String> {
        let mut result = String::new();

        let mut current_pos = self.get_pos_for_key('A').unwrap();
        for char in code.chars() {
            let next_pos = match char {
                '^' => current_pos.move_one(Direction::North),
                '>' => current_pos.move_one(Direction::East),
                'v' => current_pos.move_one(Direction::South),
                '<' => current_pos.move_one(Direction::West),
                'A' => {
                    let current_key = self.get_key_for_pos(&current_pos).unwrap();
                    result.push(current_key);

                    current_pos
                },
                _ => return None
            };

            if let None = self.get_key_for_pos(&next_pos) {
                panic!("out of bounds on '{}'", char);
            }

            current_pos = next_pos;
        }

        Some(result)
    }

    pub fn solve_path(&self, start: char, goal: char) -> Vec<Vec<Direction>> {
        if start == goal {
            return vec![vec![]];
        }

        let start_pos = self.get_pos_for_key(start).unwrap();
        
        let mut walkers = vec![KeypadWalker::new(start_pos)];
        let mut completed = vec![];

        while walkers.len() > 0 {
            let mut next_walkers = vec![];

            for walker in walkers {
                for &direction in DIRECTIONS {
                    if let Some(next_walker) = walker.move_one(direction) {
                        if let Some(next_key) = self.get_key_for_pos(next_walker.pos()) {
                            if next_key == goal {
                                completed.push(next_walker);
                            } else {
                                next_walkers.push(next_walker);
                            }
                        }
                    }                    
                }
            }

            walkers = next_walkers;
        }
        
        let best_score = completed.iter().map(|w| w.len()).min().unwrap();        

        let mut best_paths = vec![];
        for walker in completed.into_iter() {
            if walker.len() == best_score {
                best_paths.push(walker.into_path());
            }
        }

        best_paths
    }

    pub fn solve_code(&self, code: &str) -> Vec<Vec<String>> {
        let mut results = vec![];

        let mut current_char = 'A';
        for char in code.chars() {
            let mut char_results = vec![];
            for path in self.solve_path(current_char, char) {
                let subcode = path_to_code(&path);

                char_results.push(subcode);
            }

            results.push(char_results);
            current_char = char;
        }
        
        results
    }
}

#[derive(Debug)]
struct KeypadWalker {
    pos: Position,
    path: Vec<Direction>,
    visited: HashSet<Position>
}

impl KeypadWalker {
    pub fn new(pos: Position) -> KeypadWalker {
        let path = vec![];

        let mut visited = HashSet::new();
        visited.insert(pos);

        KeypadWalker { pos, path, visited }
    }
    
    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn len(&self) -> usize {
        self.path.len()
    }

    pub fn into_path(self) -> Vec<Direction> {
        self.path
    }

    pub fn move_one(&self, direction: Direction) -> Option<KeypadWalker> {
        let pos = self.pos.move_one(direction);
    
        if self.visited.contains(&pos) {
            return None;
        }

        let mut path = self.path.clone();
        path.push(direction);

        let mut visited = self.visited.clone();
        visited.insert(pos);

        Some(KeypadWalker { pos, path, visited })
    }
}

#[derive(Debug, Error)]
pub enum ParseInstructionError {
    #[error("invalid instruction")]
    InvalidInstruction
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example5() {
        solve("379A", 25);
    }

    #[test]
    pub fn simulate1() {
        let keypad = Keypad::numeric_keypad();
        let result = keypad.simulate("^AA<A").unwrap();

        assert_eq!(result, "332")
    }
}