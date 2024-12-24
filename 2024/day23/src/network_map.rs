use std::collections::{HashMap, HashSet};
use std::iter::once;
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

    pub fn clusters(&self) -> () {
        // let mut clusters = HashSet::new();

        let mut node_queue: Vec<_> = self.graph.keys().collect();
        let mut visited = HashSet::new();

        while let Some(node) = node_queue.pop() {
            if !visited.insert(node) {
                continue;
            }

            let mut cluster = HashSet::new();
            cluster.insert(node);
            
            let mut walkers = vec![NodeWalker::new(node.clone())];
            while !walkers.is_empty() {
                println!("{}", walkers.len());

                let mut next_walkers = vec![];

                for walker in walkers {
                    let neighbors = self.graph.get(&walker.node).expect("node should always be in graph");
                    for neighbor in neighbors {
                        if !walker.path.contains(neighbor) {
                            // println!("{}->{}", walker.node, neighbor);
                            next_walkers.push(walker.add_node(neighbor.clone()));
                        }
                    }
                }

                walkers = next_walkers;
            }        
        }   
        
        todo!()
    }
}

#[derive(Debug)]
struct NodeWalker {
    node: String,
    path: HashSet<String>
}

impl NodeWalker {
    pub fn new(node: String) -> NodeWalker {        
        let mut path = HashSet::new();
        path.insert(node.clone());
        
        NodeWalker { node, path }
    }

    pub fn add_node(&self, node: String) -> NodeWalker {
        let mut path = self.path.clone();
        path.insert(node.clone());

        NodeWalker { node, path }
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