use std::collections::HashMap;
use std::iter;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug)]
pub struct WarehouseSimulationSpec {
    rows: Vec<Vec<MapTile>>,
    instructions: Vec<Instruction>
}

impl WarehouseSimulationSpec {
    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}

impl FromStr for WarehouseSimulationSpec {
    type Err = ParseWarehouseSimulationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = vec![];
        let mut instructions = vec![];

        let mut lines = s.lines();

        // Grab the first line of the map
        let first_line;
        if let Some(line) = lines.next() {
            first_line = line;

            let first_row: Vec<MapTile> = iter::repeat(MapTile::Wall).take(first_line.len()).collect();
            rows.push(first_row);
        } else {
            return Err(ParseWarehouseSimulationError::UnexpectedEndOfString);
        }

        // Read rest of the map
        loop {
            if let Some(line) = lines.next() {
                if line == first_line {
                    let last_row: Vec<MapTile> = iter::repeat(MapTile::Wall).take(first_line.len()).collect();
                    rows.push(last_row);
                    
                    break;
                }
                
                let mut row = vec![MapTile::Wall];
                for char in line.chars().skip(1).take(line.len()-2) {
                    let tile = match char {
                        'O' => MapTile::Box,
                        '@' => MapTile::Robot,
                        '#' => MapTile::Wall,
                        _ => MapTile::Floor
                    };

                    row.push(tile);
                }
                row.push(MapTile::Wall);

                rows.push(row);
            } else {
                return Err(ParseWarehouseSimulationError::UnexpectedEndOfString);
            }
        }

        // Skip a line
        if let None = lines.next() {
            return Err(ParseWarehouseSimulationError::UnexpectedEndOfString);
        }

        loop {
            if let Some(line) = lines.next() {
                for char in line.chars() {
                    let instruction = match char {
                        '^' => Instruction::Up,
                        '>' => Instruction::Right,
                        'v' => Instruction::Down,
                        '<' => Instruction::Left,
                        _ => return Err(ParseWarehouseSimulationError::UnrecognizedInstruction(char))
                    };

                    instructions.push(instruction);
                }
            } else {
                break;
            }
        }

        Ok(WarehouseSimulationSpec { rows, instructions })
    }
}

#[derive(Error, Debug)]
pub enum ParseWarehouseSimulationError {
    #[error("unexpected end of string")]
    UnexpectedEndOfString,
    #[error("unrecognized tile: {0}")]
    UnrecognizedTile(char),
    #[error("unrecognized instruction: {0}")]
    UnrecognizedInstruction(char)
}

#[derive(Debug, Clone, Copy)]
enum MapTile {
    Floor,
    Wall,
    Box,
    Robot
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug)]
pub struct WarehouseSimulation {
    robot_pos: (i32, i32),
    entities: HashMap<(i32, i32), Entity>,
    width: i32,
    height: i32
}

impl WarehouseSimulation {
    pub fn from_spec(spec: &WarehouseSimulationSpec) -> WarehouseSimulation {
        let mut robot_pos = (0, 0);
        let mut entities = HashMap::new();

        let mut width: i32 = 0;
        let mut height: i32 = 0;

        for (y, row) in spec.rows.iter().enumerate() {
            let y: i32 = y.try_into().unwrap();
            height = y + 1;

            for (x, tile) in row.iter().enumerate() {
                let x: i32 = x.try_into().unwrap();
                width = x + 1;

                match tile {
                    MapTile::Floor => { },
                    MapTile::Wall => { entities.insert((x, y), Entity::Wall); },
                    MapTile::Box => { entities.insert((x, y), Entity::Box); },
                    MapTile::Robot => { robot_pos = (x, y); }
                }
            }
        }

        WarehouseSimulation { robot_pos, entities, width, height }
    }

    pub fn from_spec_doubled(spec: &WarehouseSimulationSpec) -> WarehouseSimulation {
        let mut robot_pos = (0, 0);
        let mut entities = HashMap::new();

        let mut width: i32 = 0;
        let mut height: i32 = 0;

        for (y, row) in spec.rows.iter().enumerate() {
            let y: i32 = y.try_into().unwrap();
            height = y + 1;

            for (x, tile) in row.iter().enumerate() {
                let x: i32 = (x * 2).try_into().unwrap();
                width = x + 2;

                match tile {
                    MapTile::Floor => { },
                    MapTile::Wall => { 
                        entities.insert((x, y), Entity::Wall);
                        entities.insert((x + 1, y), Entity::Wall);
                    },
                    MapTile::Box => {
                        entities.insert((x, y), Entity::LargeBoxLeft);
                        entities.insert((x + 1, y), Entity::LargeBoxRight);
                    },
                    MapTile::Robot => { robot_pos = (x, y); }
                }
            }
        }

        WarehouseSimulation { robot_pos, entities, width, height }
    }

    pub fn process_instruction(&mut self, instruction: Instruction) {
        let current_pos = self.robot_pos;
        let (next_x, next_y) = current_pos.apply_instruction(instruction);

        match self.entity_at(next_x, next_y) {
            Some(Entity::Wall) => { 
                // Do nothing
            },
            Some(Entity::Box | Entity::LargeBoxLeft | Entity::LargeBoxRight) => {
                if self.try_push_box(next_x, next_y, instruction) {
                    self.robot_pos = (next_x, next_y);
                }
            },
            None => {
                self.robot_pos = (next_x, next_y);
            }
        }
    }

    fn entity_at(&self, x: i32, y: i32) -> Option<Entity> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            self.entities.get(&(x, y)).cloned()
        }
    }

    fn try_push_box(&mut self, x: i32, y: i32, instruction: Instruction) -> bool {
        match instruction {
            Instruction::Left | Instruction::Right => self.try_push_box_left_right(x, y, instruction),
            Instruction::Up | Instruction::Down => self.try_push_box_up_down(x, y, instruction)
        }
    }

    fn try_push_box_left_right(&mut self, x: i32, y: i32, instruction: Instruction) -> bool {
        let mut current_pos = (x, y);
        let mut entities_to_move = vec![current_pos];

        loop {
            let (next_x, next_y) = current_pos.apply_instruction(instruction);
            match self.entity_at(next_x, next_y) {
                Some(Entity::Wall) => { 
                    return false;
                },
                Some(Entity::Box) => { 
                    entities_to_move.push((next_x, next_y));
                    current_pos = (next_x, next_y);
                },
                Some(Entity::LargeBoxLeft) => {
                    entities_to_move.push((next_x, next_y));
                    current_pos = (next_x, next_y);
                },
                Some(Entity::LargeBoxRight) => {
                    entities_to_move.push((next_x, next_y));
                    current_pos = (next_x, next_y);
                },
                None => {
                    while let Some(old_pos) = entities_to_move.pop() {
                        let new_pos = old_pos.apply_instruction(instruction);
                        if let Some(entity) = self.entities.remove(&old_pos) {
                            self.entities.insert(new_pos, entity);
                        }
                    }
                    return true;
                }
            }
        }
    }

    fn try_push_box_up_down(&mut self, x: i32, y: i32, instruction: Instruction) -> bool {
        let mut front_edge = match self.entity_at(x, y) {
            Some(Entity::Wall) => {
                return false;
            },
            Some(Entity::Box) => {
                vec![(x, y)]
            },
            Some(Entity::LargeBoxLeft) => {
                vec![(x, y), (x + 1, y)]
            },
            Some(Entity::LargeBoxRight) => {
                vec![(x - 1, y), (x, y)]
            },
            None => {
                return true;
            }
        };
        
        let mut entities_to_move= front_edge.clone();

        let mut entity_detected = true;
        while entity_detected {
            entity_detected = false;

            let mut next_front_edge = vec![];

            for pos in front_edge {
                let (next_x, next_y) = pos.apply_instruction(instruction);
                match self.entity_at(next_x, next_y) {
                    Some(Entity::Wall) => { 
                        // Bail out on first wall encountered
                        return false;
                    },
                    Some(Entity::Box) => { 
                        entity_detected = true;

                        entities_to_move.push((next_x, next_y));
                        next_front_edge.push((next_x, next_y));
                    },
                    Some(Entity::LargeBoxLeft) => {
                        entity_detected = true;

                        entities_to_move.push((next_x, next_y));
                        entities_to_move.push((next_x + 1, next_y));
                        
                        next_front_edge.push((next_x, next_y));
                        next_front_edge.push((next_x + 1, next_y));
                    },
                    Some(Entity::LargeBoxRight) => {
                        entity_detected = true;

                        entities_to_move.push((next_x - 1, next_y));
                        entities_to_move.push((next_x, next_y));
                        
                        next_front_edge.push((next_x - 1, next_y));
                        next_front_edge.push((next_x, next_y));
                    },
                    None => { }
                }
            }
            
            front_edge = next_front_edge;
        }

        // No more entities in the way
        while let Some(old_pos) = entities_to_move.pop() {
            let new_pos = old_pos.apply_instruction(instruction);
            if let Some(entity) = self.entities.remove(&old_pos) {
                self.entities.insert(new_pos, entity);
            }
        }

        true
    }

    pub fn score(&self) -> i32 {
        let mut score = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                score += match self.entity_at(x, y) {
                    Some(Entity::Box | Entity::LargeBoxLeft) => y * 100 + x,
                    _ => 0
                }
            }
        }

        score
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if (x, y) == self.robot_pos {
                    print!("@");
                } else {
                    match self.entity_at(x, y) {
                        Some(Entity::Wall) => { 
                            print!("#");
                        },
                        Some(Entity::Box) => { 
                            print!("O");
                        },
                        Some(Entity::LargeBoxLeft) => {
                            print!("[");
                        },
                        Some(Entity::LargeBoxRight) => {
                            print!("]");
                        },
                        None => {
                            print!(".");
                        }
                    }
                }                
            }
            println!();
        }
    }
}

trait Position {
    fn apply_instruction(&self, instruction: Instruction) -> Self;
    fn up(&self) -> Self;
    fn right(&self) -> Self;
    fn down(&self) -> Self;
    fn left(&self) -> Self;
}

impl Position for (i32, i32) { 
    fn apply_instruction(&self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Up => self.up(),
            Instruction::Right => self.right(),
            Instruction::Down => self.down(),
            Instruction::Left => self.left()
        }
    }

    fn up(&self) -> Self {
        let (x, y) = (self.0, self.1);
        (x, y - 1)
    }
    
    fn right(&self) -> Self {
        let (x, y) = (self.0, self.1);
        (x + 1, y)
    }
    
    fn down(&self) -> Self {
        let (x, y) = (self.0, self.1);
        (x, y + 1)
    }
    
    fn left(&self) -> Self {
        let (x, y) = (self.0, self.1);
        (x - 1, y)
    }
}

#[derive(Debug, Clone, Copy)]
enum Entity {
    Wall,
    Box,
    LargeBoxLeft,
    LargeBoxRight
}