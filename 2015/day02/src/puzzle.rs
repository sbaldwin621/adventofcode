use std::cmp::min;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub struct PuzzleInput {
    presents: Vec<PresentDimensions>
}

impl PuzzleInput {
    pub fn new(presents: Vec<PresentDimensions>) -> PuzzleInput {
        PuzzleInput { presents }
    }

    pub fn total_surface_area(&self) -> u32 {
        self.presents.iter().map(|p| p.surface_area()).sum()
    }

    pub fn total_paper_needed(&self) -> u32 {
        self.presents.iter().map(|p| p.surface_area() + p.extra_needed()).sum()
    }

    pub fn total_ribbon_needed(&self) -> u32 {
        self.presents.iter().map(|p| p.ribbon_needed()).sum()
    }
}

impl FromStr for PuzzleInput {
    type Err = ParsePuzzleInputError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut presents: Vec<PresentDimensions> = vec![];

        for line in s.lines() {
            if let [l, w, h] = line.split('x').collect::<Vec<_>>()[..] {
                let l = l.parse::<u32>()?;
                let w = w.parse::<u32>()?;
                let h = h.parse::<u32>()?;

                presents.push(PresentDimensions::new(l, w, h))
            } else {
                return Err(ParsePuzzleInputError::InvalidPresentFormat(line.to_string()));
            }
        }

        Ok(PuzzleInput::new(presents))
    }
}

#[derive(Debug)]
pub struct PresentDimensions {
    l: u32,
    w: u32,
    h: u32
}

impl PresentDimensions {
    pub fn new(l: u32, w: u32, h: u32) -> PresentDimensions {
        PresentDimensions { l, w, h }
    }

    pub fn top_area(&self) -> u32 {
        self.l * self.w
    }

    pub fn front_area(&self) -> u32 {
        self.w * self.h
    }

    pub fn side_area(&self) -> u32 {
        self.l * self.h
    }

    pub fn surface_area(&self) -> u32 {
        2 * self.top_area() + 2 * self.front_area() + 2 * self.side_area()
    }

    pub fn extra_needed(&self) -> u32 {
        [self.top_area(), self.front_area(), self.side_area()].into_iter().min().unwrap()
    }

    pub fn top_perimeter(&self) -> u32 {
        self.l * 2 + self.w * 2
    }

    pub fn front_perimeter(&self) -> u32 {
        self.w * 2 + self.h * 2
    }

    pub fn side_perimeter(&self) -> u32 {
        self.l * 2 + self.h * 2
    }

    pub fn smallest_perimeter(&self) -> u32 {
        [self.top_perimeter(), self.front_perimeter(), self.side_perimeter()].into_iter().min().unwrap()
    }

    pub fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }

    pub fn ribbon_needed(&self) -> u32 {
        self.smallest_perimeter() + self.volume()
    }
}

#[derive(Error, Debug)]
pub enum ParsePuzzleInputError {
    #[error("invalid present format; got {0}, expected LxWxH")]
    InvalidPresentFormat(String),
    #[error("invalid number {0}")]
    InvalidNumber(#[from]ParseIntError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_examples() {
        assert!("2x3x4".parse::<PuzzleInput>().unwrap().total_paper_needed() == 58);
        assert!("1x1x10".parse::<PuzzleInput>().unwrap().total_paper_needed() == 43);
    }

    #[test]
    pub fn part2_examples() {
        assert!("2x3x4".parse::<PuzzleInput>().unwrap().total_ribbon_needed() == 34);
        assert!("1x1x10".parse::<PuzzleInput>().unwrap().total_ribbon_needed() == 14);
    }
}
