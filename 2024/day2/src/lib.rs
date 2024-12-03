use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<i32, ApplicationError> {
    let filename = options.filename;

    let mut reports = vec![];

    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;
        
        let report = line.parse::<Report>()?;
        reports.push(report);
    }

    match options.part {
        1 => run_part1(reports),
        2 => run_part2(reports),
        _ => Err(ApplicationError::UnknownPart)
    }
}

fn run_part1(reports: Vec<Report>) -> Result<i32, ApplicationError> {
    let score: i32 = reports.iter().filter(|r| r.is_safe()).count().try_into().unwrap();

    Ok(score)
}

fn run_part2(reports: Vec<Report>) -> Result<i32, ApplicationError> {
    let score: i32 = reports.iter().filter(|r| r.is_safe_with_dampener()).count().try_into().unwrap();

    Ok(score)
}

pub struct Report {
    levels: Vec<i32>
}

impl Report {
    pub fn new(levels: Vec<i32>) -> Report {
        Report { levels }
    }

    pub fn is_safe(&self) -> bool {
        if self.levels.len() < 2 {
            return false;
        }

        let expected_polarity = (self.levels[1] - self.levels[0]).clamp(-1, 1);
        if expected_polarity == 0 {
            return false;
        }

        for i in 0..self.levels.len() - 1 {
            let a = self.levels[i];
            let b = self.levels[i + 1];

            let difference: i32 = b - a;
            let abs_difference = difference.abs();
            let polarity = difference.clamp(-1, 1);

            if abs_difference > 3 {
                return false;
            }

            if polarity != expected_polarity {
                return false;
            }
        }

        true
    }

    pub fn is_safe_with_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.levels.len() {
            let levels_without_i: Vec<i32> = self.levels
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, &level)| level)
                .collect();

            let new_report = Report::new(levels_without_i);

            if new_report.is_safe() {
                return true;
            }
        }

        false
    }
}

impl FromStr for Report {
    type Err = ParseReportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(Report { levels })
    }
}

#[derive(Error, Debug)]
pub enum ParseReportError {
    #[error("failed to parse int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error),
    #[error("invalid report: {0}")]
    ParseReportError(#[from] ParseReportError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report() {
        assert_eq!(Report::new(vec![7, 6, 4, 2, 1]).is_safe(), true);
        assert_eq!(Report::new(vec![1, 2, 7, 8, 9]).is_safe(), false);
        assert_eq!(Report::new(vec![9, 7, 6, 2, 1]).is_safe(), false);
        assert_eq!(Report::new(vec![1, 3, 2, 4, 5]).is_safe(), false);
        assert_eq!(Report::new(vec![8, 6, 4, 4, 1]).is_safe(), false);
        assert_eq!(Report::new(vec![1, 3, 6, 7, 9]).is_safe(), true);
    }

    #[test]
    fn test_report_with_dampener() {
        assert_eq!(Report::new(vec![7, 6, 4, 2, 1]).is_safe_with_dampener(), true);
        assert_eq!(Report::new(vec![1, 2, 7, 8, 9]).is_safe_with_dampener(), false);
        assert_eq!(Report::new(vec![9, 7, 6, 2, 1]).is_safe_with_dampener(), false);
        assert_eq!(Report::new(vec![1, 3, 2, 4, 5]).is_safe_with_dampener(), true);
        assert_eq!(Report::new(vec![8, 6, 4, 4, 1]).is_safe_with_dampener(), true);
        assert_eq!(Report::new(vec![1, 3, 6, 7, 9]).is_safe_with_dampener(), true);
    }
}