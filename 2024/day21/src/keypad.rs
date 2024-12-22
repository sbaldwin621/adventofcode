use std::collections::HashMap;

pub fn solve(code: &str) -> String {
    let keypads = [
        Keypad::numeric_keypad(),
        Keypad::directional_keypad(),
        Keypad::directional_keypad()
    ];

    let mut current_code = code.to_string();
    for keypad in keypads {
        current_code = keypad.solve_code(&current_code).unwrap();
    }
    
    current_code
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