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
        let mut node_to_id = HashMap::new();

        let mut keys: Vec<_> = self.graph.keys().collect();
        keys.sort();

        if keys.len() > 63 {
            panic!("keys can only be 63 long or less");
        }

        let mut n: u64 = 0;
        for key in keys {
            let id: u64 = 1 << n;
            node_to_id.insert(key.to_string(), id);

            n += 1;
        }

        let mut masks = HashMap::new();
        for (node, neighbors) in self.graph.iter() {
            let id = node_to_id[node];
            let mut mask = id;

            for neighbor in neighbors {
                let neighbor_id = node_to_id[neighbor];

                mask ^= neighbor_id;
            }

            masks.insert(node, mask);
        }

        let mut combined = HashMap::new();

        for (&node, &mask) in masks.iter() {
            for (_, &other_mask) in masks.iter() {
                let anded = mask & other_mask;
                if anded != 0 {
                    *combined.entry(anded).or_insert(0_usize) += 1;
                }
            }
        }

        let highest_count = *combined.values().max().unwrap();
        let most_common = combined.iter().filter(|(_, count)| **count == highest_count).next().unwrap().0;

        let mut nodes_in_network = vec![];
        for (node, &id) in node_to_id.iter() {
            if id & most_common == id {
                nodes_in_network.push(node);
            }
        }

        nodes_in_network.sort();

        println!("{:?}", nodes_in_network);

        todo!()
    }

    fn clusters_(&self) -> () {
        // let mut clusters = HashSet::new();

        let mut node_queue: Vec<_> = self.graph.keys().collect();
        println!("{}", node_queue.len());

        while let Some(node) = node_queue.pop() {
            let mut visited = HashSet::new();
            visited.insert(node);

            let mut cluster = HashSet::new();
            cluster.insert(node);
            
            let mut walkers = HashSet::new();
            walkers.insert(node);

            while !walkers.is_empty() {
                let mut next_walkers = HashSet::new();

                for walker in walkers {
                    let neighbors = self.graph.get(walker).expect("node should always be in graph");
                    for neighbor in neighbors {
                        if visited.insert(neighbor) {
                            let twobors = self.graph.get(neighbor).expect("neighbor should always be in graph");
                            if twobors.contains(node) {
                                cluster.insert(neighbor);
                                next_walkers.insert(neighbor);
                            }
                        }
                    }
                }

                walkers = next_walkers;
            }

            println!("{} {:?}", node, cluster);
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