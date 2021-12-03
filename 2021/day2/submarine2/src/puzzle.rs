#[derive(Debug, PartialEq, Eq)]
pub struct PuzzleInput {
    instructions: Vec<Instruction>
}

impl PuzzleInput {
    pub fn new(instructions: Vec<Instruction>) -> PuzzleInput {
        PuzzleInput { instructions }
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Forward(i64),
    Down(i64),
    Up(i64)
}

#[derive(Debug)]
pub struct Submarine {
    position: i64,
    depth: i64,
    aim: i64
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine { position: 0, depth: 0, aim: 0 }
    }

    pub fn position(&self) -> i64 {
        self.position
    }

    pub fn depth(&self) -> i64 {
        self.depth
    }

    pub fn aim(&self) -> i64 {
        self.aim
    }

    pub fn value(&self) -> i64 {
        self.position * self.depth
    }

    pub fn go(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Forward(amount) => {
                self.position += amount;
                self.depth += amount * self.aim;
            },
            Instruction::Down(amount) => self.aim += amount,
            Instruction::Up(amount) => self.aim -= amount
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let puzzle_input = PuzzleInput::new(vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2)
        ]);

        let mut submarine = Submarine::new();

        for instruction in puzzle_input.instructions().iter() {
            submarine.go(instruction);
        }
        
        assert_eq!(900, submarine.value());
    }
}