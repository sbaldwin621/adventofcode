use std::collections::HashSet;
use std::str::{FromStr, Lines};

use thiserror::Error;

#[derive(Debug)]
pub struct PuzzleInput {
    schematics: Vec<Schematic>
}

impl PuzzleInput {
    pub fn new(schematics: Vec<Schematic>) -> PuzzleInput {
        PuzzleInput { schematics }
    }

    pub fn solve(&self) -> usize {
        let keys = self.schematics.iter().filter(|s| s.is_key());
        let locks = self.schematics.iter().filter(|s| s.is_lock());

        println!("{} keys", keys.clone().count());
        println!("{} locks", locks.clone().count());

        let mut count = 0;

        for lock in locks {
            for key in keys.clone() {
                if !lock.overlaps(key) {
                    count += 1;
                }
            }
        }

        count
    }
}

impl FromStr for PuzzleInput {
    type Err = ParsePuzzleInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn read_schematic(lines: &mut Lines) -> Result<Option<Schematic>, ParsePuzzleInputError> {
            let mut grid = HashSet::new();

            let mut y = 0;
            while let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }

                for (x, char) in line.chars().enumerate() {
                    match char {
                        '#' => {
                            grid.insert((x, y));
                        },
                        '.' => { },
                        _ => return Err(ParsePuzzleInputError::UnexpectedCharacter(char))
                    }
                }

                y += 1;
            }

            if !grid.is_empty() {
                let schematic_type = match grid.get(&(0, 0)) {
                    Some(_) => SchematicType::Lock,
                    None => SchematicType::Key
                };

                Ok(Some(Schematic::new(schematic_type, grid)))
            } else {
                Ok(None)
            }
        }

        let mut schematics = vec![];
        let mut lines = s.lines();

        while let Some(schematic) = read_schematic(&mut lines)? {
            schematics.push(schematic);
        }

        Ok(PuzzleInput::new(schematics))
    }
}

#[derive(Debug, Error)]
pub enum ParsePuzzleInputError {
    #[error("unexpected character: {0}")]
    UnexpectedCharacter(char)
}

#[derive(Debug)]
pub struct Schematic {
    schematic_type: SchematicType,
    grid: HashSet<(usize, usize)>
}

impl Schematic {
    pub fn new(schematic_type: SchematicType, grid: HashSet<(usize, usize)>) -> Schematic {
        Schematic { schematic_type, grid }
    }

    pub fn is_lock(&self) -> bool {
        matches!(self.schematic_type, SchematicType::Lock)
    }

    pub fn is_key(&self) -> bool {
        matches!(self.schematic_type, SchematicType::Key)
    }

    pub fn overlaps(&self, other: &Schematic) -> bool {
        !self.grid.is_disjoint(&other.grid)
    }
}

#[derive(Debug)]
pub enum SchematicType {
    Key,
    Lock
}