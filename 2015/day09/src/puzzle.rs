use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;
use std::usize;

use itertools::Itertools;
use thiserror::Error;

#[derive(Debug)]
pub struct PuzzleInput {
    locations: Vec<String>,
    edges: HashMap<String, HashMap<String, usize>>
}

impl PuzzleInput {
    pub fn shortest_distance(&self) -> usize {
        let mut shortest = usize::MAX;
        for permutation in self.locations.iter().permutations(self.locations.len()) {
            if let Some(distance) = self.distance_for_route(&permutation) {
                if distance < shortest {
                    shortest = distance;
                }
            }            
        }

        shortest
    }

    pub fn longest_distance(&self) -> usize {
        let mut longest = 0;
        for permutation in self.locations.iter().permutations(self.locations.len()) {
            if let Some(distance) = self.distance_for_route(&permutation) {
                if distance > longest {
                    longest = distance;
                }
            }            
        }

        longest
    }

    fn distance_for_route(&self, route: &Vec<&String>) -> Option<usize> {
        let mut total_distance = 0;
        for window in route.windows(2) {
            if let [start, end] = window {
                if let Some(distance) = self.distance_between(start, end) {
                    total_distance += distance;
                } else {
                    return None;
                }                    
            }
        }

        Some(total_distance)
    }

    fn distance_between(&self, start: &str, end: &str) -> Option<usize> {
        self.edges.get(start)?.get(end).copied()
    }
}

impl FromStr for PuzzleInput {
    type Err = ParsePuzzleInputError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut edges: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut locations = HashSet::new();
        for line in s.lines() {
            let edge = line.parse::<Edge>()?;
            
            if !locations.contains(&edge.start) {
                locations.insert(edge.start.clone());
            }

            if !locations.contains(&edge.end) {
                locations.insert(edge.end.clone());
            }

            edges.entry(edge.start.clone()).or_default().insert(edge.end.clone(), edge.distance);

            // Done with these values, so we can move them instead of cloning
            edges.entry(edge.end).or_default().insert(edge.start, edge.distance);
        }

        let locations = locations.into_iter().collect();

        Ok(PuzzleInput { locations, edges })
    }
}

#[derive(Error, Debug)]
pub enum ParsePuzzleInputError {
    #[error("invalid edge: {0}")]
    InvalidEdge(#[from] ParseEdgeError)
}

#[derive(Debug)]
struct Edge {
    start: String,
    end: String,
    distance: usize
}

impl Edge {
    pub fn invert(&self) -> Edge {
        Edge { start: self.end.clone(), end: self.start.clone(), distance: self.distance }
    }
}

impl FromStr for Edge {
    type Err = ParseEdgeError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((locations, distance)) = s.split_once(" = ") {
            if let Some((start, end)) = locations.split_once(" to ") {
                let distance = distance.parse::<usize>()?;
                let start = start.to_owned();
                let end = end.to_owned();

                Ok(Edge { start, end, distance })
            } else {
                Err(ParseEdgeError::InvalidLocationPair)
            }
        } else {
            Err(ParseEdgeError::InvalidEdge)
        }
    }    
}


#[derive(Error, Debug)]
pub enum ParseEdgeError {
    #[error("invalid edge")]
    InvalidEdge,
    #[error("invalid location pair")]
    InvalidLocationPair,
    #[error("invalid distance: {0}")]
    InvalidDistance(#[from]ParseIntError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let example_input = concat!(
            "London to Dublin = 464\n",
            "London to Belfast = 518\n",
            "Dublin to Belfast = 141"
        ).parse::<PuzzleInput>().unwrap();

        assert_eq!(example_input.shortest_distance(), 605);
    }
}
