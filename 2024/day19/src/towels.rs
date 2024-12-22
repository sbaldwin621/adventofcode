use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use thiserror::Error;

pub struct TowelSolver<'a> {
    towels: HashSet<&'a String>,
    orders: &'a Vec<String>,
    solved_orders: HashMap<String, Option<Vec<Vec<String>>>>
}

impl<'a> TowelSolver<'a> {
    pub fn new(towels: &'a Vec<String>, orders: &'a Vec<String>) -> TowelSolver<'a> {
        let mut solved_orders = HashMap::new();
        for towel in towels.iter() {
            solved_orders.insert(towel.clone(), Some(vec![vec![towel.clone()]]));
        }

        let towels: HashSet<_> = towels.iter().collect();
        
        TowelSolver { towels, orders, solved_orders }
    }

    pub fn solve(&mut self) -> Vec<&'a String> {
        // let mut result = vec![];

        for order in self.orders.iter() {
            let solutions = self.solve_order(order);
            println!("{} final: {:?}", order, solutions);
        }
        
        todo!()
    }

    fn solve_order(&mut self, order: &str) -> Option<Vec<Vec<String>>> {
        if let Some(answer) = self.solved_orders.get(order) {
            return answer.clone();
        }

        if order.len() == 1 {
            return None;
        }

        let mut solutions = vec![];

        for n in 1..order.len() {
            let (left, right) = order.split_at(n);

            println!("{}: {} | {}", order, left, right);

            if let Some(left_solutions) = self.solve_order(left) {
                if let Some(right_solutions) = self.solve_order(right) {
                    for left_solution in left_solutions {
                        for right_solution in right_solutions.iter() {
                            let mut right_solution = right_solution.clone();
                            let mut solution = left_solution.clone();
                            solution.append(&mut right_solution);

                            solutions.push(solution);
                        }
                    }
                }
            }
        }

        println!("{}: {:?}", order, solutions);

        if solutions.len() > 0 {
            self.solved_orders.insert(order.to_string(), Some(solutions.clone()));
            return Some(solutions);
        } else {
            self.solved_orders.insert(order.to_string(), None);
            return None;
        }        
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