use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub struct DebuggerInfo {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<Instruction>
}

impl FromStr for DebuggerInfo {
    type Err = ParseDebuggerInfoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_register_line(register: &str, line: Option<&str>) -> Result<usize, ParseDebuggerInfoError> {
            if let Some(line) = line {
                let pattern = "Register ".to_string() + register + ": ";
                if let Some(value) = line.trim().strip_prefix(&pattern) {
                    let value: usize = value.trim().parse()?;
                    
                    Ok(value)
                } else {
                    Err(ParseDebuggerInfoError::InvalidSyntax)
                }
            } else {
                Err(ParseDebuggerInfoError::UnexpectedEndOfString)
            }
        }

        fn parse_program_line(line: Option<&str>) -> Result<Vec<usize>, ParseDebuggerInfoError> {
            if let Some(line) = line {
                if let Some(line) = line.trim().strip_prefix("Program: ") {
                    let values: Result<Vec<_>, _> = line.split(',')
                        .map(|value| value.trim().parse::<usize>())
                        .collect();
                    
                    Ok(values?)
                } else {
                    Err(ParseDebuggerInfoError::InvalidSyntax)
                }
            } else {
                Err(ParseDebuggerInfoError::UnexpectedEndOfString)
            }
        }

        fn parse_combo_operand(operand: usize) -> Result<ComboOperand, ParseDebuggerInfoError> {
            match operand {
                0..=3 => Ok(ComboOperand::Literal(operand)),
                4 => Ok(ComboOperand::RegisterA),
                5 => Ok(ComboOperand::RegisterB),
                6 => Ok(ComboOperand::RegisterC),
                _ => Err(ParseDebuggerInfoError::UnrecognizedComboOperand(operand))
            }
        }

        fn parse_instruction(opcode: usize, operand: usize) -> Result<Instruction, ParseDebuggerInfoError> {
            match opcode {
                0 => Ok(Instruction::Adv(parse_combo_operand(operand)?)),
                1 => Ok(Instruction::Bxl(operand)),
                2 => Ok(Instruction::Bst(parse_combo_operand(operand)?)),
                3 => Ok(Instruction::Jnz(operand)),
                4 => Ok(Instruction::Bxc),
                5 => Ok(Instruction::Out(parse_combo_operand(operand)?)),
                6 => Ok(Instruction::Bdv(parse_combo_operand(operand)?)),
                7 => Ok(Instruction::Cdv(parse_combo_operand(operand)?)),
                _ => Err(ParseDebuggerInfoError::UnrecognizedOpcode(opcode))
            }
        }

        let mut lines = s.lines();
        
        let register_a = parse_register_line("A", lines.next())?;
        let register_b = parse_register_line("B", lines.next())?;
        let register_c = parse_register_line("C", lines.next())?;

        // Skip a line
        lines.next();

        let program_bytes = parse_program_line(lines.next())?;

        let mut program = vec![];
        for chunk in program_bytes.chunks_exact(2) {
            if let [opcode, operand] = chunk {
                let instruction = parse_instruction(*opcode, *operand)?;
                program.push(instruction);
            }
        }

        Ok(DebuggerInfo { register_a, register_b, register_c, program })
    }
}

#[derive(Debug, Error)]
pub enum ParseDebuggerInfoError {
    #[error("unexpected end of string")]
    UnexpectedEndOfString,
    #[error("invalid syntax")]
    InvalidSyntax,
    #[error("couldn't parse integer: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("unrecognized opcode: {0}")]
    UnrecognizedOpcode(usize),
    #[error("unrecognized combo operand: {0}")]
    UnrecognizedComboOperand(usize)
}

#[derive(Debug, Clone)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(usize),
    Bst(ComboOperand),
    Jnz(usize),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand)
}

#[derive(Debug, Clone, Copy)]
enum ComboOperand {
    Literal(usize),
    RegisterA,
    RegisterB,
    RegisterC
}

pub struct Emulator {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    program: Vec<Instruction>,
    instruction_counter: usize,
    output_buffer: Vec<usize>
}

impl Emulator {
    pub fn from_debugger_info(debugger_info: &DebuggerInfo) -> Emulator {
        let register_a = debugger_info.register_a;
        let register_b = debugger_info.register_b;
        let register_c = debugger_info.register_c;
        let program = debugger_info.program.clone();
    
        let instruction_counter = 0;
        let output_buffer = vec![];

        Emulator { register_a, register_b, register_c, program, instruction_counter, output_buffer }
    }

    pub fn output_buffer(&self) -> &Vec<usize> {
        &self.output_buffer
    }

    pub fn step(&mut self) -> bool {
        if let Some(instruction) = self.instruction_at(self.instruction_counter) {
            match instruction {
                Instruction::Adv(operand) => self.adv(operand),
                Instruction::Bxl(operand) => self.bxl(operand),
                Instruction::Bst(operand) => self.bst(operand),
                Instruction::Jnz(operand) => self.jnz(operand),
                Instruction::Bxc => self.bxc(),
                Instruction::Out(operand) => self.out(operand),
                Instruction::Bdv(operand) => self.bdv(operand),
                Instruction::Cdv(operand) => self.cdv(operand)
            }

            true
        } else {
            false
        }
    }

    fn instruction_at(&self, location: usize) -> Option<Instruction> {
        let index = location / 2;

        self.program.get(index).cloned()
    }

    fn eval_operand(&self, operand: ComboOperand) -> usize {
        match operand {
            ComboOperand::Literal(literal) => literal,
            ComboOperand::RegisterA => self.register_a,
            ComboOperand::RegisterB => self.register_b,
            ComboOperand::RegisterC => self.register_c
        }
    }

    fn eval_denominator(&self, operand: ComboOperand) -> usize {
        2usize.pow(self.eval_operand(operand).try_into().unwrap())
    }

    fn adv(&mut self, operand: ComboOperand) {
        let numerator = self.register_a;
        let denominator = self.eval_denominator(operand);

        self.register_a = numerator / denominator;

        self.instruction_counter += 2;
    }

    fn bxl(&mut self, operand: usize) {
        self.register_b ^= operand;

        self.instruction_counter += 2;
    }

    fn bst(&mut self, operand: ComboOperand) {
        self.register_b = self.eval_operand(operand) % 8;

        self.instruction_counter += 2;
    }

    fn jnz(&mut self, operand: usize) {
        if self.register_a != 0 {
            self.instruction_counter = operand;
        } else {
            self.instruction_counter += 2;
        }
    }

    fn bxc(&mut self) {
        self.register_b ^= self.register_c;

        self.instruction_counter += 2;
    }

    fn out(&mut self, operand: ComboOperand) {
        let value = self.eval_operand(operand) % 8;
        self.output_buffer.push(value);
        
        self.instruction_counter += 2;
    }

    fn bdv(&mut self, operand: ComboOperand) {
        let numerator = self.register_a;
        let denominator = self.eval_denominator(operand);

        self.register_b = numerator / denominator;

        self.instruction_counter += 2;
    }

    fn cdv(&mut self, operand: ComboOperand) {
        let numerator = self.register_a;
        let denominator = self.eval_denominator(operand);

        self.register_c = numerator / denominator;

        self.instruction_counter += 2;
    }
}

fn run_program(program: &str) -> Result<Emulator, ParseDebuggerInfoError> {
    let debugger_info: DebuggerInfo = program.trim().parse()?;
    let mut emulator = Emulator::from_debugger_info(&debugger_info);

    while emulator.step() { }

    Ok(emulator)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example1() {
        let result = run_program("
            Register A: 0
            Register B: 0
            Register C: 9
            
            Program: 2,6
        ").unwrap();

        assert_eq!(result.register_b, 1);
    }

    #[test]
    pub fn example2() {
        let result = run_program("
            Register A: 10
            Register B: 0
            Register C: 0
            
            Program: 5,0,5,1,5,4
        ").unwrap();

        assert_eq!(result.output_buffer, vec![0, 1, 2]);
    }

    #[test]
    pub fn example3() {
        let result = run_program("
            Register A: 2024
            Register B: 0
            Register C: 0
            
            Program: 0,1,5,4,3,0
        ").unwrap();

        assert_eq!(result.register_a, 0);
        assert_eq!(result.output_buffer, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    pub fn example4() {
        let result = run_program("
            Register A: 0
            Register B: 29
            Register C: 0
            
            Program: 1,7
        ").unwrap();

        assert_eq!(result.register_b, 26);
    }

    #[test]
    pub fn example5() {
        let result = run_program("
            Register A: 0
            Register B: 2024
            Register C: 43690
            
            Program: 4,0
        ").unwrap();

        assert_eq!(result.register_b, 44354);
    }
}