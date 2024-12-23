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

    pub fn clusters(&self) -> HashSet<[&String;3]> {
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