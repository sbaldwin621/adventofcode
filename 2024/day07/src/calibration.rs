use std::iter;
use std::num::ParseIntError;
use std::str::FromStr;

use itertools::{repeat_n, Itertools};
use thiserror::Error;

pub struct UnsolvedCalibrationSet {
    calibrations: Vec<UnsolvedCalibration>
}

impl UnsolvedCalibrationSet {
    pub fn new(calibrations: Vec<UnsolvedCalibration>) -> UnsolvedCalibrationSet {
        UnsolvedCalibrationSet { calibrations }
    }

    pub fn calculate_score(&self, operator_set: &Vec<Operator>) -> u64 {
        let mut score = 0;

        for calibration in self.calibrations.iter() {
            if calibration.is_possible(operator_set) {
                score += calibration.test_value;
            }
        }

        score
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnsolvedCalibration {
    test_value: u64,
    operands: Vec<u64>
}

impl UnsolvedCalibration {
    pub fn new(test_value: u64, operands: Vec<u64>) -> UnsolvedCalibration {
        UnsolvedCalibration { test_value, operands }
    }

    pub fn is_possible(&self, operator_set: &Vec<Operator>) -> bool {
        for operators in self.all_operator_combinations(operator_set) {
            let value = self.apply_operators(&operators);
            if value == self.test_value {
                return true;
            }
        }

        false
    }

    fn all_operator_combinations<'a>(&self, operator_set: &'a Vec<Operator>) -> itertools::MultiProduct<std::slice::Iter<'a, Operator>> {
        let n = self.operands.len() - 1;
        
        repeat_n(operator_set.iter(), n)
            .multi_cartesian_product()
    }

    fn apply_operators(&self, operators: &Vec<&Operator>) -> u64 {
        let mut accum = 0;
        for (i, operand) in self.operands.iter().enumerate() {
            if i == 0 {
                accum = *operand;
            } else {
                match operators[i - 1] {
                    Operator::Add => accum += operand,
                    Operator::Multiply => accum *= operand,
                    Operator::Concatenate => {
                        let operand_digits = operand.checked_ilog10().unwrap_or(0) + 1;
                        accum = accum * (10 as u64).pow(operand_digits) + operand;
                    },
                }
            }            
        }

        accum
    }
}

impl FromStr for UnsolvedCalibration {
    type Err = UnsolvedCalibrationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 8: 1 2 3 4

        if let Some((test_value, rest)) = s.split_once(':') {
            let test_value: u64 = test_value.parse()?;
            
            let mut operands = vec![];
            for operand in rest.split_whitespace() {
                let operand: u64 = operand.parse()?;
                operands.push(operand);
            }

            Ok(UnsolvedCalibration::new(test_value, operands))
        } else {
            Err(UnsolvedCalibrationParseError::InvalidFormat(s.into()))
        }
    }
}

#[derive(Debug, Error)]
pub enum UnsolvedCalibrationParseError {
    #[error("line was not in the correct format: {0}")]
    InvalidFormat(String),
    #[error("integer was not in the correct format: {0}")]
    InvalidInteger(#[from] ParseIntError)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_unsolved_calibration() {
        let calibration: UnsolvedCalibration = "8: 1 2 3 4".parse().unwrap();

        assert_eq!(calibration, UnsolvedCalibration::new(8, vec![1, 2, 3, 4]))
    }

    #[test]
    pub fn apply_operators() {
        let calibration: UnsolvedCalibration = "8: 1 2 3 4".parse().unwrap();

        assert_eq!(calibration.apply_operators(&vec![&Operator::Add, &Operator::Add, &Operator::Add]), 10);
        assert_eq!(calibration.apply_operators(&vec![&Operator::Multiply, &Operator::Multiply, &Operator::Multiply]), 24);
        assert_eq!(calibration.apply_operators(&vec![&Operator::Add, &Operator::Multiply, &Operator::Add]), 13);
        assert_eq!(calibration.apply_operators(&vec![&Operator::Concatenate, &Operator::Multiply, &Operator::Add]), 40);
    }

    #[test]
    pub fn apply_operators_2() {
        let calibration: UnsolvedCalibration = "7290: 6 8 6 15".parse().unwrap();

        assert_eq!(calibration.apply_operators(&vec![&Operator::Multiply, &Operator::Concatenate, &Operator::Multiply]), 7290);
    }

    #[test]
    pub fn is_possible_part1_yes() {
        let part1_operators = vec![Operator::Add, Operator::Multiply];

        let one: UnsolvedCalibration = "292: 11 6 16 20".parse().unwrap();
        let two: UnsolvedCalibration = "3267: 81 40 27".parse().unwrap();
        
        assert!(one.is_possible(&part1_operators));
        assert!(two.is_possible(&part1_operators));
    }

    #[test]
    pub fn is_possible_part2_yes() {
        let part2_operators = vec![Operator::Add, Operator::Multiply, Operator::Concatenate];

        let one: UnsolvedCalibration = "7290: 6 8 6 15".parse().unwrap();
        let two: UnsolvedCalibration = "192: 17 8 14".parse().unwrap();
        
        assert!(one.is_possible(&part2_operators));
        assert!(two.is_possible(&part2_operators));
    }

    #[test]
    pub fn is_possible_part1_no() {
        let part1_operators = vec![Operator::Add, Operator::Multiply];

        let one: UnsolvedCalibration = "161011: 16 10 13 ".parse().unwrap();
        let two: UnsolvedCalibration = "21037: 9 7 18 13".parse().unwrap();
          
        assert_eq!(one.is_possible(&part1_operators), false);
        assert_eq!(two.is_possible(&part1_operators), false);
    }
}