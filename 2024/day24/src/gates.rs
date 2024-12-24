use std::collections::HashMap;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub struct PuzzleInput {
    initial_values: HashMap<String, WireValue>,
    gates: Vec<Gate>
}

impl PuzzleInput {
    pub fn new(initial_values: HashMap<String, WireValue>, gates: Vec<Gate>) -> PuzzleInput {
        PuzzleInput { initial_values, gates }
    }
}

impl FromStr for PuzzleInput {
    type Err = ParsePuzzleInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut initial_values = HashMap::new();
        let mut gates = vec![];

        let mut lines = s.lines();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            
            if let Some((gate, value)) = line.split_once(": ") {
                let value: WireValue = value.parse()?;

                initial_values.insert(gate.to_string(), value);
            }
        }

        while let Some(line) = lines.next() {
            let gate: Gate = line.parse()?;
            gates.push(gate);
        }

        Ok(PuzzleInput::new(initial_values, gates))
    }
}

#[derive(Error, Debug)]
pub enum ParsePuzzleInputError {
    #[error("invalid wire value: {0}")]
    InvalidWireValue(#[from] ParseWireValueError),
    #[error("invalid gate: {0}")]
    InvalidGate(#[from] ParseGateError)
}

#[derive(Debug)]
pub enum WireValue {
    Undecided,
    True,
    False
}

impl FromStr for WireValue {
    type Err = ParseWireValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(WireValue::False),
            "1" => Ok(WireValue::True),
            _ => Err(ParseWireValueError::UnknownWireValue(s.to_string()))
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseWireValueError {
    #[error("unknown wire value: {0}")]
    UnknownWireValue(String)
}

#[derive(Debug)]
pub struct Gate {
    left_input: String,
    right_input: String,
    output: String,
    operation: GateOperation
}

impl Gate {
    pub fn new(left_input: String, right_input: String, output: String, operation: GateOperation) -> Gate {
        Gate { left_input, right_input, output, operation }
    }
}

impl FromStr for Gate {
    type Err = ParseGateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.splitn(5, ' ').collect();
        match split[..] {
            [left, op, right, "->", output] => {
                let op: GateOperation = op.parse()?;

                let left = left.to_string();
                let right = right.to_string();
                let output = output.to_string();

                Ok(Gate::new(left, right, output, op))
            }
            _ => Err(ParseGateError::InvalidSyntax)
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseGateError {
    #[error("invalid operation: {0}")]
    InvalidOperation(#[from] ParseGateOperationError),
    #[error("invalid syntax")]
    InvalidSyntax
}

#[derive(Debug)]
pub enum GateOperation {
    And,
    Or,
    Xor
}

impl FromStr for GateOperation {
    type Err = ParseGateOperationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(GateOperation::And),
            "OR" => Ok(GateOperation::Or),
            "XOR" => Ok(GateOperation::Xor),
            _ => Err(ParseGateOperationError::UnknownOperation(s.to_string()))
        }
    }
}


#[derive(Error, Debug)]
pub enum ParseGateOperationError {
    #[error("unknown operation: {0}")]
    UnknownOperation(String)
}