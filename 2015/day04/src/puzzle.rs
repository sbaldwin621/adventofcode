use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub struct PuzzleInput {
    secret_key: String
}

impl PuzzleInput {
    pub fn new(secret_key: String) -> PuzzleInput {
        PuzzleInput { secret_key }
    }

    pub fn mine(&self, num_zeroes: usize) -> u32 {
        let target = (0..num_zeroes).map(|_| "0").collect::<String>();
        
        for n in 0.. {
            let hash_input = format!("{}{}", self.secret_key, n);
            let digest = md5::compute(hash_input.as_bytes());
            let hash_as_string = format!("{:x}", digest);

            if hash_as_string.starts_with(&target) {
                return n;
            }
        }

        unreachable!()
    }
}

impl FromStr for PuzzleInput {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PuzzleInput::new(s.to_string()))
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
