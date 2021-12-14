use std::collections::HashSet;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub struct OctopiGrid {
    octopi: Vec<u32>,
    width: usize
}

impl OctopiGrid {
    pub fn new(octopi: Vec<u32>, width: usize) -> OctopiGrid {
        OctopiGrid { octopi, width }
    }

    pub fn step(&mut self) -> u64 {
        self.increment();

        let mut flashes = HashSet::new();
        loop {
            let mut flashes_this_loop = 0;

            for i in 0..self.octopi.len() {
                let has_flashed = flashes.contains(&i);
                let octopus = self.octopi[i];

                if octopus > 9 && !has_flashed {
                    flashes.insert(i);
                    flashes_this_loop += 1;

                    self.increment_around(i);
                }
            }

            if flashes_this_loop == 0 {
                break;
            }
        }

        self.reset_flashed();

        flashes.len() as u64
    }

    fn increment(&mut self) {
        for octopus in self.octopi.iter_mut() {
            *octopus += 1;
        }
    }

    fn increment_around(&mut self, i: usize) {
        let x: i64 = (i % self.width).try_into().unwrap();
        let y: i64 = (i / self.width).try_into().unwrap();

        self.increment_at(x - 1, y - 1);    
        self.increment_at(x, y - 1);    
        self.increment_at(x + 1, y - 1);    
        self.increment_at(x - 1, y);    
        self.increment_at(x + 1, y);    
        self.increment_at(x - 1, y + 1);    
        self.increment_at(x, y + 1);
        self.increment_at(x + 1, y + 1);
    }

    fn increment_at(&mut self, x: i64, y: i64) {
        let width: i64 = self.width.try_into().unwrap();
        if x >= 0 && x < width {
            let i = y * width + x;
            if let Ok(i) = usize::try_from(i) {
                if let Some(octopus) = self.octopi.get_mut(i) {
                    *octopus += 1;
                }
            }
        }
    }

    fn reset_flashed(&mut self) {
        for octopus in self.octopi.iter_mut() {
            if *octopus > 9 {
                *octopus = 0;
            }
        }
    }
}

impl Display for OctopiGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, octopus) in self.octopi.iter().enumerate() {
            if i != 0 && i % self.width == 0 {
                writeln!(f)?;
            }

            write!(f, "{}", octopus)?;
        }

        writeln!(f)
    }
}

impl FromStr for OctopiGrid {
    type Err = ParseOctopiGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut octopi = vec![];
        let mut width = None;

        for line in s.lines() {
            let len = line.len();
            match width {
                None => { width = Some(len); }
                Some(width) if width != len => {
                    return Err(ParseOctopiGridError::InconsistentWidth);
                }
                _ => { }
            }

            for char in line.chars() {
                let n = char.to_string().parse::<u32>()?;
                octopi.push(n);
            }
        }

        match width {
            None => Err(ParseOctopiGridError::NoLines),
            Some(width) => Ok(OctopiGrid::new(octopi, width))
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseOctopiGridError {
    #[error("failed to parse int: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("width not uniform")]
    InconsistentWidth,
    #[error("no lines")]
    NoLines
}