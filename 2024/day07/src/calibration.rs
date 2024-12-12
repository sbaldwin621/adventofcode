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
}