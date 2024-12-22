use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use thiserror::Error;

pub struct TowelSolver<'a> {
    towels: HashSet<&'a String>,
    orders: &'a Vec<String>,
    solved_orders: HashMap<String, bool>
}

impl<'a> TowelSolver<'a> {
    pub fn new(towels: &'a Vec<String>, orders: &'a Vec<String>) -> TowelSolver<'a> {
        let mut solved_orders = HashMap::new();
        for towel in towels.iter() {
            solved_orders.insert(towel.clone(), true);
        }

        let towels: HashSet<_> = towels.iter().collect();
        
        TowelSolver { towels, orders, solved_orders }
    }

    pub fn solve(&mut self) -> Vec<&'a String> {
        let mut result = vec![];

        for order in self.orders.iter() {
            if self.solve_order(order) {
                result.push(order);
            }
        }

        result
    }

    fn solve_order(&mut self, order: &str) -> bool {
        if let Some(answer) = self.solved_orders.get(order) {
            return *answer;
        }

        if order.len() == 1 {
            return false;
        }

        for n in 1..order.len() {
            let (left, right) = order.split_at(n);
            if self.solve_order(left) && self.solve_order(right) {
                self.solved_orders.insert(order.to_string(), true);
                return true;
            }
        }

        self.solved_orders.insert(order.to_string(), false);
        return false;
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct PuzzleInput {
    towels: Vec<String>,
    orders: Vec<String>
}

impl PuzzleInput {
    pub fn to_solver(&self) -> TowelSolver {
        TowelSolver::new(&self.towels, &self.orders)
    }
}

impl FromStr for PuzzleInput {
    type Err = ParsePuzzleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {


        let mut lines = s.lines();

        let towels: Vec<String> = if let Some(line) = lines.next() {
            line.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            return Err(ParsePuzzleError::UnexpectedEndOfString);
        };

        // Skip blank line
        lines.next();

        let orders: Vec<String> = lines.map(|s| s.to_string()).collect();

        Ok(PuzzleInput { towels, orders })
    }
}

#[derive(Debug, Error)]
pub enum ParsePuzzleError {
    #[error("unexpected end of string")]
    UnexpectedEndOfString
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_puzzle_input() {
        let puzzle_input: PuzzleInput = concat!(
            "r, wr, b, g, bwu, rb, gb, br\n",
            "\n",
            "brwrr\n",
            "bggr\n",
            "gbbr\n",
            "rrbgbr\n",
            "ubwu\n",
            "bwurrg\n",
            "brgr\n",
            "bbrgwb"
        ).parse().unwrap();

        assert_eq!(puzzle_input, PuzzleInput {
            towels: ["r", "wr", "b", "g", "bwu", "rb", "gb", "br"].iter().map(|s| s.to_string()).collect(),
            orders: ["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"].iter().map(|s| s.to_string()).collect()
        });
    }
}