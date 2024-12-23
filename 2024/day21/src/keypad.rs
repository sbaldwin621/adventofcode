use std::collections::HashMap;
use std::str::FromStr;

use thiserror::Error;

pub fn solve(code: &str) -> usize {
    let keypads = [
        Keypad::numeric_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad()
    ];

    let mut current_code = code.to_string();
    for keypad in keypads {
        current_code = keypad.solve_code(&current_code).unwrap();
    }
    
    let value = code_numeric_value(code).unwrap();
    let complexity = current_code.len() * value;

    println!("{}: {}", code, current_code);
    println!("{}: {} * {} = {}", code, current_code.len(), value, complexity);

    complexity
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
                return None;
            }

            current_pos = next_pos;
        }

        Some(result)
    }

    pub fn solve_path(&self, start: char, goal: char) -> Option<Vec<Direction>> {
        if start == goal {
            return Some(vec![]);
        }

        let start_pos = self.get_pos_for_key(start)?;
        
        let mut best_scores = HashMap::new();

        let mut walkers = vec![KeypadWalker::new(start_pos)];
        let mut completed = vec![];

        while walkers.len() > 0 {
            let mut next_walkers = vec![];

            for walker in walkers {
                for &direction in DIRECTIONS {
                    let next_walker = walker.move_one(direction);
                    if let Some(next_key) = self.get_key_for_pos(next_walker.pos()) {
                        let best_score = best_scores.get(next_walker.pos()).cloned().unwrap_or(usize::MAX);
                        if next_walker.score() < best_score {
                            best_scores.insert(*next_walker.pos(), next_walker.score());

                            if next_key == goal {
                                completed.push(next_walker.into_path());
                            } else {
                                next_walkers.push(next_walker);
                            }
                        }
                    }
                }
            }

            walkers = next_walkers;
        }

        completed.pop()
    }

    pub fn solve_code(&self, code: &str) -> Option<String> {
        let mut result = String::new();
        
        let mut current_char = 'A';
        for char in code.chars() {
            let path = self.solve_path(current_char, char)?;
            let subcode = path_to_code(&path);
            
            result.push_str(&subcode);

            current_char = char;
        }

        Some(result)
    }
}

#[derive(Debug)]
struct KeypadWalker {
    pos: Position,
    path: Vec<Direction>
}

impl KeypadWalker {
    pub fn new(pos: Position) -> KeypadWalker {
        let path = vec![];

        KeypadWalker { pos, path }
    }
    
    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn score(&self) -> usize {
        self.path.len()
    }

    pub fn into_path(self) -> Vec<Direction> {
        self.path
    }

    pub fn move_one(&self, direction: Direction) -> KeypadWalker {
        let mut path = self.path.clone();
        path.push(direction);

        let pos = self.pos.move_one(direction);

        KeypadWalker { pos, path }
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
    use super::Keypad;

    #[test]
    pub fn simulate1() {
        let keypad = Keypad::numeric_keypad();
        let result = keypad.simulate("^AA<A").unwrap();

        assert_eq!(result, "332")
    }

    #[test]
    pub fn simulate2() {
        let keypad = Keypad::numeric_keypad();
        let result = keypad.simulate(&keypad.solve_code("357").unwrap()).unwrap();

        assert_eq!(result, "357")
    }


    #[test]
    pub fn simulate3() {
        let keypad = Keypad::numeric_keypad();
        let keypad2 = Keypad::directional_keypad();
        let keypad3 = Keypad::directional_keypad();

        let code1 = keypad.solve_code("029A").unwrap();
        let code2 = keypad2.solve_code(&code1).unwrap();
        let code3 = keypad3.solve_code(&code2).unwrap();

        println!("{}", code3);

        let result = keypad.simulate(
            &keypad2.simulate(
                &keypad3.simulate(
                    &code3
                ).unwrap()
            ).unwrap()
        ).unwrap();

        println!("{:?}", result);
    }
}