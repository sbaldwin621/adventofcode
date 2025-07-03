use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub struct PuzzleInput {
}

impl FromStr for PuzzleInput {
    type Err = ParsePuzzleInputError;
    
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum ParsePuzzleInputError {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        todo!()
    }
}
