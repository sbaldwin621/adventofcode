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
        self.is_possible_inner(operator_set, 0, 0)
    }
    
    fn is_possible_inner(&self, operator_set: &Vec<Operator>, n: usize, accum: u64) -> bool {
        if accum > self.test_value {
            return false;
        }

        if n == self.operands.len() {
            return accum == self.test_value;
        }

        let operand = self.operands[n];
        for operator in operator_set.iter() {
            let next = match operator {
                Operator::Add => accum + operand,
                Operator::Multiply => accum * operand,
                Operator::Concatenate => {
                    let operand_digits = operand.checked_ilog10().unwrap_or(0) + 1;
                    accum * (10 as u64).pow(operand_digits) + operand
                }
            };

            if self.is_possible_inner(operator_set, n + 1, next) {
                return true;
            }
        }
        
        false
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
}