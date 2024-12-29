use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::once;
use std::str::FromStr;
use std::usize;

use itertools::{chain, Itertools};
use thiserror::Error;

pub fn solve(code: &str, directional_keypad_count: usize) -> usize {
    let mut keypads = vec![
        Keypad::numeric_keypad()
    ];

    for _ in 0..directional_keypad_count {
        keypads.push(Keypad::directional_keypad());
    }

    let mut current_codes = vec![code.to_string()];
    
    for (n, keypad) in keypads.iter().enumerate() {
        println!("keypad {}: {:?}", n, current_codes.iter().take(1).collect::<Vec<_>>()[0].len());

        let mut solutions = vec![];
        for code in current_codes {
            let solution = keypad.solve_code(&code);
            solutions.push(solution);
        }

        let lowest_cost = solutions.iter().map(|s| s.cost).min().unwrap();
        let best_solutions = solutions.iter().filter(|s| s.cost == lowest_cost);
        let next_codes: Vec<_> = best_solutions.flat_map(|s| s.codes()).collect();

        current_codes = next_codes;
    }

    let shortest_code = current_codes.iter().next().unwrap();

    let value = code_numeric_value(code).unwrap();
    let complexity = shortest_code.len() * value;

    println!("{}: {}", code, shortest_code);
    println!("{}: {} * {} = {}", code, shortest_code.len(), value, complexity);

    complexity
}

fn keep_shortest(set: HashSet<String>) -> HashSet<String> {
    let shortest_len = set.iter().map(|c| c.len()).min().unwrap();
    let result: HashSet<_> = set.into_iter().filter(|c| c.len() == shortest_len).collect();

    result
}

fn cost_of_path(path: &str) -> usize {
    let mut total_cost = 0;

    for (a, b) in chain!(once('A'), path.chars()).tuple_windows() {
        let cost = match (a, b) {
            ('A', '^') => 2,
            ('A', '>') => 2,
            ('A', 'v') => 3,
            ('A', '<') => 4,
            ('A', 'A') => 1,
            ('^', '^') => 1,
            ('^', '>') => 3,
            ('^', 'v') => 2,
            ('^', '<') => 3,
            ('^', 'A') => 2,
            ('>', '^') => 3,
            ('>', '>') => 1,
            ('>', 'v') => 2,
            ('>', '<') => 3,
            ('>', 'A') => 2,
            ('v', '^') => 2,
            ('v', '>') => 2,
            ('v', 'v') => 1,
            ('v', '<') => 2,
            ('v', 'A') => 3,
            ('<', '^') => 3,
            ('<', '>') => 3,
            ('<', 'v') => 2,
            ('<', '<') => 1,
            ('<', 'A') => 4,
            _ => panic!("{} -> {}", a, b)
        };

        total_cost += cost;
    }

    total_cost
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

fn code_to_instructions(code: &str) -> Result<Vec<Instruction>, ParseInstructionError> {
    let mut result = vec![];

    for char in code.chars() {
        let instruction: Instruction = char.to_string().parse()?;
        result.push(instruction);
    }

    Ok(result)
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
        let goal_pos = self.get_pos_for_key(goal).unwrap();
        
        // let mut best_scores = HashMap::new();

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
        
        let best_score = completed.iter().map(|w| w.final_score()).min().unwrap();        

        let mut best_paths = vec![];
        for walker in completed.into_iter() {
            if walker.final_score() == best_score {
                // println!("{:?}", walker);
                best_paths.push(walker.into_path());
            }
        }

        best_paths
    }

    pub fn solve_code(&self, code: &str) -> CodeSolution {
        // println!("solve code {}", code);
        
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
        
        CodeSolution::new(results)
    }
}

#[derive(Debug)]
struct CodeSolution {
    results: Vec<Vec<String>>,
    cost: usize
}

impl CodeSolution {
    pub fn new(results: Vec<Vec<String>>) -> CodeSolution {
        let mut first_code = String::new();

        for segments in results.iter() {
            first_code.push_str(&segments.iter().next().unwrap());
        }

        let cost = cost_of_path(&first_code);

        CodeSolution { results, cost }
    }

    pub fn codes(&self) -> Vec<String> {
        let mut current_codes = vec![String::new()];

        for segments in self.results.iter() {
            let mut next_codes = vec![];

            for segment in segments.iter() {
                for code in current_codes.iter() {
                    let mut builder = code.clone();
                    builder.push_str(&segment);

                    next_codes.push(builder);
                }
            }

            current_codes = next_codes;
        }

        current_codes
    }
}

#[derive(Debug)]
struct KeypadWalker {
    pos: Position,
    facing: Option<Direction>,
    path: Vec<Direction>,
    visited: HashSet<Position>,
    score: usize
}

impl KeypadWalker {
    pub fn new(pos: Position) -> KeypadWalker {
        let facing = None;
        let path = vec![];
        let score = 0;

        let mut visited = HashSet::new();
        visited.insert(pos);

        KeypadWalker { pos, facing, path, visited, score }
    }
    
    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn final_score(&self) -> usize {
        let final_move_cost = match self.facing {
            Some(Direction::North) => 2,
            Some(Direction::East)  => 2,
            Some(Direction::South) => 3,
            Some(Direction::West)  => 4,
            None => 0,
        };

        // let final_move_cost = 0;

        self.score + final_move_cost
    }

    pub fn into_path(self) -> Vec<Direction> {
        self.path
    }

    pub fn move_one(&self, direction: Direction) -> Option<KeypadWalker> {
        let pos = self.pos.move_one(direction);
    
        if self.visited.contains(&pos) {
            return None;
        }

        // let cost = match self.facing {
        //     None => 1,
        //     Some(facing) if facing == direction => 1,
        //     _ => 2
        // };

        let cost = match (self.facing, direction) {
            (None,                   Direction::North) => 2,
            (None,                   Direction::East)  => 2,
            (None,                   Direction::South) => 3,
            (None,                   Direction::West)  => 4,
            (Some(Direction::North), Direction::North) => 1,
            (Some(Direction::North), Direction::East)  => 3,
            (Some(Direction::North), Direction::South) => return None,
            (Some(Direction::North), Direction::West)  => 3,
            (Some(Direction::East),  Direction::North) => 3,
            (Some(Direction::East),  Direction::East)  => 1,
            (Some(Direction::East),  Direction::South) => 2,
            (Some(Direction::East),  Direction::West)  => return None,
            (Some(Direction::South), Direction::North) => return None,
            (Some(Direction::South), Direction::East)  => 2,
            (Some(Direction::South), Direction::South) => 1,
            (Some(Direction::South), Direction::West)  => 2,
            (Some(Direction::West),  Direction::North) => 3,
            (Some(Direction::West),  Direction::East)  => return None,
            (Some(Direction::West),  Direction::South) => 2,
            (Some(Direction::West),  Direction::West)  => 1
        };

        let score = self.score + cost;

        let mut path = self.path.clone();
        path.push(direction);

        let mut visited = self.visited.clone();
        visited.insert(pos);

        let facing = Some(direction);

        Some(KeypadWalker { pos, facing, path, visited, score })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Instruction {
    Move(Direction),
    PressButton
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Instruction::Move(Direction::North)),
            ">" => Ok(Instruction::Move(Direction::East)),
            "v" => Ok(Instruction::Move(Direction::South)),
            "<" => Ok(Instruction::Move(Direction::West)),
            "A" => Ok(Instruction::PressButton),
            _ => Err(ParseInstructionError::InvalidInstruction)
        }
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
    pub fn cost() {
        assert_eq!(cost_of_path("<A") + cost_of_path(">A<AAv<AA>>^AvAA^Av<AAA^>A"), cost_of_path("<A>A<AAv<AA>>^AvAA^Av<AAA^>A"));
    }

    #[test]
    pub fn example5() {
        solve("379A", 2);
    }

    #[test]
    pub fn simulate1() {
        let keypad = Keypad::numeric_keypad();
        let result = keypad.simulate("^AA<A").unwrap();

        assert_eq!(result, "332")
    }

    // #[test]
    // pub fn simulate2() {
    //     let keypad = Keypad::numeric_keypad();
    //     let result = keypad.simulate(&keypad.solve_code("357").unwrap()).unwrap();

    //     assert_eq!(result, "357")
    // }


    // #[test]
    // pub fn simulate3() {
    //     let keypad = Keypad::numeric_keypad();
    //     let keypad2 = Keypad::directional_keypad();
    //     let keypad3 = Keypad::directional_keypad();

    //     let code1 = keypad.solve_code("379A").unwrap();
    //     let code2 = keypad2.solve_code(&code1).unwrap();
    //     let code3 = keypad3.solve_code(&code2).unwrap();

    //     println!("{}", code1);
    //     println!("{}", code2);
    //     println!("{}", code3);

    //     let result = keypad.simulate(
    //         &keypad2.simulate(
    //             &keypad3.simulate(
    //                 &code3
    //             ).unwrap()
    //         ).unwrap()
    //     ).unwrap();

    //     println!("{:?}", result);
    // }

    // #[test]
    // pub fn example5() {
    //     let result = simulate("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A").unwrap();

    //     assert_eq!(result, "379A");
    // }
}