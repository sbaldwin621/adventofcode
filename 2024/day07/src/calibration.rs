use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

pub struct UnsolvedCalibrationSet {
    calibrations: Vec<UnsolvedCalibration>
}

impl UnsolvedCalibrationSet {
    pub fn new(calibrations: Vec<UnsolvedCalibration>) -> UnsolvedCalibrationSet {
        UnsolvedCalibrationSet { calibrations }
    }

    pub fn calculate_score(&self) -> u64 {
        let mut score = 0;

        for calibration in self.calibrations.iter() {
            if calibration.is_possible() {
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

    pub fn is_possible(&self) -> bool {
        let upper_operator_bound = (2 as u32).pow((self.operands.len() - 1) as u32);

        for operator in 0..=upper_operator_bound {
            let value = self.apply_operators(operator);
            if value == self.test_value {
                return true;
            }
        }

        false
    }

    fn apply_operators(&self, operator_value: u32) -> u64 {
        let mut accum = 0;
        for (i, operand) in self.operands.iter().enumerate() {
            if i == 0 {
                accum = *operand;
            } else {
                let current_operator = operator_value >> (i - 1);
                if (current_operator & 1) == 1 {
                    accum *= operand;
                } else {
                    accum += operand;
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

        assert_eq!(calibration.apply_operators(0b000), 10);
        assert_eq!(calibration.apply_operators(0b111), 24);
        assert_eq!(calibration.apply_operators(0b010), 13);
    }

    #[test]
    pub fn is_possible_yes() {
        let one: UnsolvedCalibration = "292: 11 6 16 20".parse().unwrap();
        let two: UnsolvedCalibration = "3267: 81 40 27".parse().unwrap();
        
        assert_eq!(one.is_possible(), true);
        assert_eq!(two.is_possible(), true);
    }

    #[test]
    pub fn is_possible_no() {
        let one: UnsolvedCalibration = "161011: 16 10 13 ".parse().unwrap();
        let two: UnsolvedCalibration = "21037: 9 7 18 13".parse().unwrap();
          
        assert_eq!(one.is_possible(), false);
        assert_eq!(two.is_possible(), false);
    }
}