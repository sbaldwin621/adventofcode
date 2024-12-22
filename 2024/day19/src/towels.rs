use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use thiserror::Error;

pub struct TowelSolver<'a> {
    towels: HashSet<String>,
    largest_towel: usize,
    orders: &'a Vec<String>
}

impl<'a> TowelSolver<'a> {
    pub fn new(towels: &'a Vec<String>, orders: &'a Vec<String>) -> TowelSolver<'a> {       
        let towels: HashSet<_> = towels.iter().cloned().collect();
        let largest_towel = towels.iter().max_by_key(|t| t.len()).unwrap().len();
        
        TowelSolver { towels, largest_towel, orders }
    }

    pub fn solve(&mut self) -> Vec<CompletedOrder> {
        let mut result = vec![];

        for order in self.orders.iter() {
            let combinations = self.solve_order(order);
            result.push(CompletedOrder { order: order.to_string(), combinations });
        }

        result
    }

    fn solve_order(&self, order: &str) -> Vec<Vec<String>> {
        let mut walkers = vec![Walker::new(0, vec![])];
        
        let mut completed_paths = vec![];

        while walkers.len() > 0 {
            let mut next_walkers = vec![];

            for walker in walkers {
                if walker.i == order.len() {
                    completed_paths.push(walker.path);
                    continue;
                }

                for n in 1..=self.largest_towel {
                    let i = walker.i;
                    if i + n > order.len() {
                        break;
                    }

                    let potential_towel = &order[i..i+n];
                    if self.towels.contains(potential_towel) {
                        let new_walker = walker.add_towel(potential_towel);
                        next_walkers.push(new_walker);
                    }
                }
            }

            println!("{} walkers", next_walkers.len());

            walkers = next_walkers;
        }

        completed_paths
    }    
}

#[derive(Debug)]
pub struct CompletedOrder {
    order: String,
    combinations: Vec<Vec<String>>
}

impl CompletedOrder {
    pub fn order(&self) -> &str {
        &self.order
    }

    pub fn combinations(&self) -> &Vec<Vec<String>> {
        &self.combinations
    }

    pub fn is_possible(&self) -> bool {
        self.combinations.len() > 0
    }
}

#[derive(Debug, Hash)]
struct Walker {
    i: usize,
    path: Vec<String>
}

impl Walker {
    pub fn new(i: usize, path: Vec<String>) -> Walker {
        Walker { i, path }
    }

    pub fn add_towel(&self, towel: &str) -> Walker {
        let i = self.i + towel.len();
        let mut path = self.path.clone();
        path.push(towel.to_string());
        
        Walker { i, path }
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