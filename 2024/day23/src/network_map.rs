use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use thiserror::Error;

pub struct NetworkMap {
    graph: HashMap<String, HashSet<String>>
}

impl NetworkMap {
    pub fn new(graph: HashMap<String, HashSet<String>>) -> NetworkMap {
        NetworkMap { graph }
    }

    pub fn clusters_of_three(&self) -> HashSet<[&String;3]> {
        let mut clusters = HashSet::new();

        for (computer, neighbors) in self.graph.iter() {
            for neighbor in neighbors.iter() {
                if let Some(twobors) = self.graph.get(neighbor) {
                    for twobor in twobors.iter() {
                        if let Some(threebors) = self.graph.get(twobor) {
                            if threebors.contains(computer) {
                                let mut cluster = [computer, neighbor, twobor];
                                cluster.sort();

                                clusters.insert(cluster);
                            }
                        }
                    }
                }
            }
        }

        clusters
    }

    pub fn largest_network(&self) -> Vec<String> {
        let mut masks = HashMap::new();
        for (node, neighbors) in self.graph.iter() {
            let mut mask = neighbors.clone();
            mask.insert(node.clone());

            masks.insert(node, mask);
        }

        let mut combined = HashMap::new();
        for (_, mask) in masks.iter() {
            for (_, other_mask) in masks.iter() {
                let mut intersection: Vec<_> = mask.intersection(other_mask).cloned().collect();
                intersection.sort_unstable();

                if !intersection.is_empty() {
                    
                    *combined.entry(intersection).or_insert(0_usize) += 1;
                }
            }
        }

        let most_common = combined.iter().max_by_key(|(_, count)| **count).unwrap().0;
        most_common.clone()
    }
}

impl FromStr for NetworkMap {
    type Err = ParseNetworkMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = HashMap::new();

        for line in s.lines() {
            if let Some((left, right)) = line.split_once('-') {
                let left_entry = graph.entry(left.to_string()).or_insert_with(|| HashSet::new());
                left_entry.insert(right.to_string());

                let right_entry = graph.entry(right.to_string()).or_insert_with(|| HashSet::new());
                right_entry.insert(left.to_string());
            } else {
                return Err(ParseNetworkMapError::InvalidSyntax);
            }
        }

        Ok(NetworkMap::new(graph))
    }
}

#[derive(Debug, Error)]
pub enum ParseNetworkMapError {
    #[error("invalid syntax")]
    InvalidSyntax
}