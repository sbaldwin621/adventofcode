use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug)]
pub struct PuzzleInput {
    statements: Vec<Statement>
}

impl PuzzleInput {
    pub fn new(statements: Vec<Statement>) -> PuzzleInput {
        PuzzleInput { statements }
    }

    pub fn eval(&self) -> Option<u16> {
        let mut eval_context = EvaluationContext::new(&self.statements);

        eval_context.eval_statement("a")
    }

    pub fn eval_two(&self) -> Option<u16> {
        let mut eval_context = EvaluationContext::new(&self.statements);

        let a = eval_context.eval_statement("a")?;

        eval_context.reset();
        eval_context.set("b", a);

        eval_context.eval_statement("a")
    }
}

impl FromStr for PuzzleInput {
    type Err = ParsePuzzleInputError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut statements = Vec::new();

        for line in s.lines() {
            println!("{:?}", line);

            let statement = line.parse::<Statement>()?;
            statements.push(statement);
        }

        Ok(PuzzleInput::new(statements))
    }
}

#[derive(Error, Debug)]
pub enum ParsePuzzleInputError {
    #[error("invalid statement: {0}")]
    ParseStatementError(#[from] ParseStatementError)
}

struct EvaluationContext<'a> {
    statements: &'a Vec<Statement>,
    values: HashMap<String, u16>
}

impl<'a> EvaluationContext<'a> {
    pub fn new(statements: &'a Vec<Statement>) -> EvaluationContext {
        let values = HashMap::new();
        EvaluationContext { statements, values }
    }

    pub fn reset(&mut self) {
        self.values.clear();
    }

    pub fn set(&mut self, wire: &str, value: u16) {
        self.values.insert(wire.to_owned(), value);
    }

    pub fn eval_statement(&mut self, wire: &str) -> Option<u16> {
        if let Some(&value) = self.values.get(wire) {
            Some(value)
        } else {
            let statement_for_wire = self.statements.iter().find(|s| s.output == wire);
            if let Some(statement) = statement_for_wire {
                println!("{:?}", statement);

                let result = self.eval_expression(&statement.expression)?;

                self.values.insert(wire.to_owned(), result);

                Some(result)
            } else {
                None
            }

        }
    }

    fn eval_expression(&mut self, expression: &Expression) -> Option<u16> {
        match expression {
            Expression::Literal(value) => Some(*value),
            Expression::Reference(wire) => self.eval_statement(wire),
            Expression::And(left, right) => {
                let left = self.eval_statement(left)?;
                let right = self.eval_statement(right)?;

                Some(left & right)
            },
            Expression::AndLiteral(left, right) => {
                let left = self.eval_statement(left)?;

                Some(left & right)
            },
            Expression::Or(left, right) => {
                let left = self.eval_statement(left)?;
                let right = self.eval_statement(right)?;

                Some(left | right)
            },
            Expression::Not(wire) => {
                let value = self.eval_statement(wire)?;

                Some(!value)
            },
            Expression::LShift(wire, amount) => {
                let value = self.eval_statement(wire)?;

                Some(value << amount)
            },
            Expression::RShift(wire, amount) => {
                let value = self.eval_statement(wire)?;

                Some(value >> amount)
            },
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(u16),
    Reference(String),
    And(String, String),
    AndLiteral(String, u16),
    Or(String, String),
    Not(String),
    LShift(String, u8),
    RShift(String, u8)
}

impl FromStr for Expression {
    type Err = ParseExpressionError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" ")
            .collect::<Vec<&str>>();

        let expression = match split[..] {
            [value] => {
                if let Ok(number) = value.parse::<u16>() {
                    Expression::Literal(number)
                } else {
                    Expression::Reference(value.to_owned())
                }
            },
            [left, "AND", right] => {
                if let Ok(number) = left.parse::<u16>() {
                    Expression::AndLiteral(right.trim().to_owned(), number)
                } else if let Ok(number) = right.parse::<u16>() {
                    Expression::AndLiteral(left.trim().to_owned(), number)
                } else {
                    Expression::And(left.trim().to_owned(), right.trim().to_owned())
                }
            },
            [left, "OR", right] => Expression::Or(left.trim().to_owned(), right.trim().to_owned()),
            ["NOT", operand] => Expression::Not(operand.trim().to_owned()),
            [gate, "LSHIFT", amount] => Expression::LShift(gate.trim().to_owned(), amount.parse::<u8>()?),
            [gate, "RSHIFT", amount] => Expression::RShift(gate.trim().to_owned(), amount.parse::<u8>()?),
            _ => return Err(ParseExpressionError::UnknownOperator)
        };

        Ok(expression)
    }
}

#[derive(Error, Debug)]
pub enum ParseExpressionError {
    #[error("unknown operator")]
    UnknownOperator,
    #[error("couldn't parse int: {0}")]
    ParseIntError(#[from] ParseIntError)
}

#[derive(Debug)]
pub struct Statement {
    output: String,
    expression: Expression
}

impl FromStr for Statement {
    type Err = ParseStatementError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" -> ")
            .collect::<Vec<&str>>();

        let statement = match split[..] {
            [left, right] => {
                let operator = left.parse::<Expression>()?;

                Statement { output: right.trim().to_owned(), expression: operator }
            },
            _ => return Err(ParseStatementError::InvalidGate)
        };

        Ok(statement)
    }
}

#[derive(Error, Debug)]
pub enum ParseStatementError {
    #[error("invalid gate")]
    InvalidGate,
    #[error("couldn't parse expression: {0}")]
    ParseExpressionError(#[from] ParseExpressionError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        todo!()
    }
}
