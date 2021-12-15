use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct CaveSystem {
    caves: HashMap<String, Cave>,
    connections: HashMap<String, Vec<String>>
}

impl CaveSystem {
    pub fn new() -> CaveSystem {
        let caves = HashMap::new();
        let connections = HashMap::new();

        CaveSystem { caves, connections }
    }

    pub fn add_connection(&mut self, from: &String, to: &String) {
        self.add_cave(from);
        self.add_cave(to);

        self.add_connection_single(from, to);
        self.add_connection_single(to, from);
    }

    fn add_cave(&mut self, id: &String) {
        if !self.caves.contains_key(id) {
            self.caves.insert(id.clone(), Cave::new(id.clone()));
        }
    }

    fn add_connection_single(&mut self, from: &String, to: &String) {
        let from = from.clone();
        let to = to.clone();

        let connections = self.connections.entry(from).or_insert_with(|| vec![]);
        connections.push(to);
    }

    pub fn paths(&self) -> Vec<Vec<String>> {
        let mut paths = vec![];

        let mut visited = HashSet::new();
        let mut path = vec![];

        self.path_helper(&"start".to_string(), &mut visited, &mut path, &mut paths);

        paths
    }

    fn path_helper(&self, current: &String, visited: &mut HashSet<String>, path: &mut Vec<String>, paths: &mut Vec<Vec<String>>) {
        visited.insert(current.clone());
        path.push(current.clone());

        if current == "end" {
            paths.push(path.clone());
        } else {
            if let Some(connections) = self.connections.get(current) {
                for next in connections.iter() {
                    let next_cave = self.caves.get(next).unwrap();
                    if next_cave.is_big || !visited.contains(next) {
                        self.path_helper(next, visited, path, paths);
                    }
                }
            }
        }

        path.pop();
        visited.remove(current);
    }
}

#[derive(Debug)]
pub struct Cave {
    id: String,
    is_big: bool
}

impl Cave {
    pub fn new(id: String) -> Cave {
        let is_big = id.chars().nth(0).unwrap().is_uppercase();
        
        Cave { id, is_big }
    }
}

#[derive(Debug)]
pub struct Connection {
    from: String,
    to: String
}
