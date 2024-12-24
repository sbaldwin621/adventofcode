use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub struct Device {
    wires: HashSet<String>,
    values: HashMap<String, bool>,
    gates_by_output: HashMap<String, Gate>
}

impl Device {
    pub fn new(values: HashMap<String, bool>, gates: Vec<Gate>) -> Device {
        let mut wires = HashSet::new();

        for (wire, _) in values.iter() {
            wires.insert(wire.to_string());
        }
        
        let mut gates_by_output = HashMap::new();

        for gate in gates {
            wires.insert(gate.left_input.to_string());
            wires.insert(gate.right_input.to_string());
            wires.insert(gate.output.to_string());

            gates_by_output.insert(gate.output.to_string(), gate);
        }

        Device { wires, values, gates_by_output }
    }

    pub fn solve(&mut self) -> usize {
        let all_z_wires: Vec<_> = self.wires.iter().filter(|w| w.starts_with('z')).collect();
        let mut current_wires = all_z_wires.clone();

        while let Some(wire) = current_wires.pop() {
            if !self.values.contains_key(wire) {
                let gate = self.gates_by_output.get(wire).expect("wire should either have a value or a gate");
                match self.evaluate_gate(gate) {
                    Some(value) => {
                        self.values.insert(wire.to_string(), value);
                    },
                    None => {
                        current_wires.push(wire);
                        current_wires.push(&gate.left_input);
                        current_wires.push(&gate.right_input);
                    }
                }
            }
        }

        self.wireset_value("z")
    }

    fn evaluate_gate(&self, gate: &Gate) -> Option<bool> {
        let left = self.values.get(&gate.left_input)?;
        let right = self.values.get(&gate.right_input)?;

        let output = match (left, gate.operation, right) {
            (true, GateOperation::And, true) => true,
            (true, GateOperation::Or, _) | (_, GateOperation::Or, true) => true,
            (true, GateOperation::Xor, false) | (false, GateOperation::Xor, true) => true,
            _ => false
        };

        Some(output)
    }

    pub fn expected_output(&self) -> usize {
        let xs_value = self.wireset_value("x");
        let ys_value = self.wireset_value("y");

        xs_value + ys_value
    }

    fn wireset_value(&self, prefix: &str) -> usize {
        let mut result = 0;

        for wire in self.wires.iter().filter(|w| w.starts_with(prefix)) {
            let value = self.values.get(wire).expect("wire must have a value");
            if *value {
                let n: u32 = wire[1..].parse().expect("wire must end in integer");
                let value = 2_usize.pow(n);

                result += value;
            }
        }

        result
    }
}

#[derive(Debug)]
pub struct PuzzleInput {
    initial_values: HashMap<String, bool>,
    gates: Vec<Gate>
}

impl PuzzleInput {
    pub fn new(initial_values: HashMap<String, bool>, gates: Vec<Gate>) -> PuzzleInput {
        PuzzleInput { initial_values, gates }
    }

    pub fn into_device(self) -> Device {
        Device::new(self.initial_values, self.gates)
    }

    pub fn gates(&self) -> &Vec<Gate> {
        &self.gates
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
                let value = match value {
                    "0" => false,
                    "1" => true,
                    _ => return Err(ParsePuzzleInputError::InvalidWireValue(s.to_string()))
                };

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
    InvalidWireValue(String),
    #[error("invalid gate: {0}")]
    InvalidGate(#[from] ParseGateError)
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

    pub fn left_input(&self) -> &str {
        &self.left_input
    }

    pub fn right_input(&self) -> &str {
        &self.right_input
    }

    pub fn output(&self) -> &str {
        &self.output
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

#[derive(Debug, Clone, Copy)]
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