use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::str::FromStr;

use itertools::chain;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Device {
    wires: HashSet<String>,
    values: HashMap<String, bool>,
    gates: Vec<Rc<RefCell<Gate>>>,
    gates_by_output: HashMap<String, Rc<RefCell<Gate>>>,
    gates_by_input: HashMap<String, Vec<Rc<RefCell<Gate>>>>
}

impl Device {
    pub fn new(values: HashMap<String, bool>, gates: Vec<Gate>) -> Device {
        let mut wires = HashSet::new();

        for (wire, _) in values.iter() {
            wires.insert(wire.to_string());
        }
        
        let mut gates_by_output = HashMap::new();
        let mut gates_by_input = HashMap::new();

        let gates: Vec<Rc<RefCell<Gate>>> = gates.into_iter().map(|g| Rc::new(RefCell::new(g))).collect();

        for gate in &gates {
            wires.insert(gate.borrow().left_input.to_string());
            wires.insert(gate.borrow().right_input.to_string());
            wires.insert(gate.borrow().output.to_string());

            gates_by_input.entry(gate.borrow().left_input.to_string()).or_insert_with(|| vec![]).push(Rc::clone(gate));
            gates_by_input.entry(gate.borrow().right_input.to_string()).or_insert_with(|| vec![]).push(Rc::clone(gate));

            gates_by_output.insert(gate.borrow().output.to_string(), Rc::clone(gate));
        }

        Device { wires, values, gates, gates_by_output, gates_by_input }
    }

    pub fn get_value(&self, wire: &str) -> Option<bool> {
        self.values.get(wire).cloned()
    }

    pub fn values_mut(&mut self) -> &mut HashMap<String, bool> {
        &mut self.values
    }

    pub fn wires(&self) -> &HashSet<String> {
        &self.wires
    }

    pub fn wires_with_prefix(&self, prefix: &str) -> Vec<&String> {
        self.wires.iter().filter(|w| w.starts_with(prefix)).collect()
    }

    pub fn swap_wires(&mut self, wire_one: &str, wire_two: &str) {
        let gate_one = self.gates_by_output.get(wire_one).unwrap();
        let gate_two = self.gates_by_output.get(wire_two).unwrap();

        gate_one.borrow_mut().output = wire_two.to_string();
        gate_two.borrow_mut().output = wire_one.to_string();
    }

    pub fn find_output(&self, input_one: &str, input_two: &str, operation: GateOperation) -> Option<String> {
        let matching_gate = self.gates.iter()
            .filter(|gate| ((gate.borrow().left_input == input_one && gate.borrow().right_input == input_two) ||
                (gate.borrow().left_input == input_two && gate.borrow().right_input == input_one))
                && gate.borrow().operation == operation)
            .next();

        matching_gate.map(|gate| gate.borrow().output().to_string())
    }

    pub fn solve(&mut self) -> DeviceOutput {
        let all_z_wires: Vec<_> = self.wires.iter().filter(|w| w.starts_with('z')).cloned().collect();
        let mut current_wires = all_z_wires;

        while let Some(wire) = current_wires.pop() {
            if !self.values.contains_key(&wire) {
                let gate = self.gates_by_output.get(&wire).expect("wire should either have a value or a gate");
                match self.evaluate_gate(&gate.borrow()) {
                    Some(value) => {
                        self.values.insert(wire.to_string(), value);
                    },
                    None => {
                        current_wires.push(wire);
                        current_wires.push(gate.borrow().left_input.to_string());
                        current_wires.push(gate.borrow().right_input.to_string());
                    }
                }
            }
        }

        self.wireset_value("z")
    }

    pub fn find_upstream(&self, target_wire: &str, depth_limit: usize) -> HashSet<String> {
        let mut connections = HashSet::new();

        let mut wire_stack = vec![(target_wire.to_string(), depth_limit)];
        while let Some((wire, depth)) = wire_stack.pop() {
            connections.insert(wire.to_string());

            if depth > 0 {
                if let Some(gate) = self.gates_by_output.get(&wire) {
                    wire_stack.push((gate.borrow().left_input().to_string(), depth - 1));
                    wire_stack.push((gate.borrow().right_input().to_string(), depth - 1));
                }
            }            
        }

        connections
    }

    pub fn find_downstream(&self, target_wire: &str, depth_limit: usize) -> HashSet<String> {
        let mut connections = HashSet::new();

        let mut wire_stack = vec![(target_wire.to_string(), depth_limit)];
        while let Some((wire, depth)) = wire_stack.pop() {
            connections.insert(wire.to_string());

            if depth > 0 {
                if let Some(gates) = self.gates_by_input.get(&wire) {
                    for gate in gates {
                        wire_stack.push((gate.borrow().output().to_string(), depth - 1));
                    }
                }    
            }            
        }

        connections
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

    pub fn expected_output(&self) -> DeviceOutput {
        let xs_value = self.wireset_value("x");
        let ys_value = self.wireset_value("y");

        let zs_value = xs_value.to_usize() + ys_value.to_usize();
        
        DeviceOutput::from_usize(zs_value)
    }

    fn wireset_value(&self, prefix: &str) -> DeviceOutput {
        let mut result = HashMap::new();
        
        for wire in self.wires.iter().filter(|w| w.starts_with(prefix)) {
            let value = self.values.get(wire).expect("wire must have a value");
            result.insert(wire.to_string(), *value);
        }

        DeviceOutput::new(result)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeviceOutput {
    values: HashMap<String, bool>
}

impl DeviceOutput {
    pub fn new(values: HashMap<String, bool>) -> DeviceOutput {
        DeviceOutput { values }
    }

    pub fn from_usize(numeric_value: usize) -> DeviceOutput {
        let mut values = HashMap::new();

        let mut numeric_value = numeric_value;
        let mut n = 0;
        while numeric_value != 0 {
            let bool = numeric_value & 1 == 1;
            let wire = format!("z{:0>2}", n).to_string();

            values.insert(wire, bool);

            n += 1;

            numeric_value >>= 1;
        }

        DeviceOutput::new(values)
    }
    
    pub fn get(&self, wire: &str) -> Option<bool> {
        self.values.get(wire).cloned()
    }

    pub fn to_usize(&self) -> usize {
        let mut result = 0;

        for (wire, value) in self.values.iter() {
            if *value {
                let n: u32 = wire[1..].parse().expect("wire must end in integer");
                let value = 2_usize.pow(n);

                result += value;
            }
        }

        result
    }

    pub fn difference(&self, other: &DeviceOutput) -> DeviceOutput {
        let mut difference = HashMap::new();

        for (wire, value) in self.values.iter() {
            let other_value = other.get(wire).expect("wires must be present in both outputs");
            difference.insert(wire.to_string(), value ^ other_value);
        }

        DeviceOutput::new(difference)
    }

    pub fn trues(&self) -> HashSet<String> {
        self.values.iter()
            .filter_map(|(w, v)| v.then(|| w.to_string()))
            .collect()
    }

    pub fn falses(&self) -> HashSet<String> {
        self.values.iter()
            .filter_map(|(w, v)| (!v).then(|| w.to_string()))
            .collect()
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

    pub fn to_device(&self) -> Device {
        Device::new(self.initial_values.clone(), self.gates.clone())
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

#[derive(Debug, Clone)]
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

    pub fn operation(&self) -> GateOperation {
        self.operation
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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