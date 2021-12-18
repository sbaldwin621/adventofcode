use std::collections::{HashMap, HashSet, BinaryHeap};

#[derive(Debug)]
pub struct MapBuilder {
    values: Vec<u64>,
    width: Option<usize>
}

impl MapBuilder {
    pub fn new() -> MapBuilder {
        let lines = vec![];
        let width = None;

        MapBuilder { values: lines, width }
    }

    pub fn add_line(&mut self, line: String) {
        if self.width == None {
            self.width = Some(line.len());
        }

        for value in line.chars().map(|c| c.to_string().parse::<u64>().unwrap()) {
            self.values.push(value);
        }
    }

    pub fn to_map(&self) -> Map {
        let width = self.width.unwrap();

        let mut nodes = vec![];
        let mut edges = HashMap::new();

        for (i, &current_cost) in self.values.iter().enumerate() {
            let x: i64 = (i % width).try_into().unwrap();
            let y: i64 = (i / width).try_into().unwrap();

            let current = (x, y);

            let north = (x, y - 1);
            let west = (x - 1, y);
            let east = (x + 1, y);
            let south = (x, y + 1);

            for next in [north, east, south, west] {
                if let Some(cost) = self.get(next) {
                    edges.entry(current).or_insert_with(|| vec![])
                        .push(Edge { from: current, to: next, cost });
                    edges.entry(next).or_insert_with(|| vec![])
                        .push(Edge { from: next, to: current, cost: current_cost });
                }
            }

            nodes.push(current);
        }

        Map { nodes, edges }
    }

    fn get(&self, pos: (i64, i64)) -> Option<u64> {
        let (x, y) = pos;
        if x < 0 || y < 0 || x >= self.width.unwrap().try_into().unwrap() {
            None
        } else {
            let y: usize = y.try_into().unwrap();
            let x: usize = x.try_into().unwrap();
            let i: usize = (y * self.width.unwrap()) + x;
            
            if let Some(&element) = self.values.get(i) {
                Some(element)
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct Edge {
    from: (i64, i64),
    to: (i64, i64),
    cost: u64
}

#[derive(Debug)]
pub struct Map {
    nodes: Vec<(i64, i64)>,
    edges: HashMap<(i64, i64), Vec<Edge>>
}

impl Map {
    pub fn corner(&self) -> (i64, i64) {
        *self.nodes.last().unwrap()
    }

    pub fn shortest_path(&self, start: (i64, i64), goal: (i64, i64)) -> Option<u64> {
        let mut dist = HashMap::new();
        for &node in self.nodes.iter() {
            dist.insert(node, u64::MAX);
        }

        let mut heap = BinaryHeap::new();

        dist.insert(start, 0);
        heap.push(PathFinderState { node: start, cost: 0 });

        while let Some(PathFinderState { node, cost }) = heap.pop() {
            if node == goal {
                return Some(cost);
            }

            if cost > dist[&node] {
                continue;
            }

            if let Some(neighbors) = self.edges.get(&node) {
                for edge in neighbors.iter() {
                    let next = PathFinderState { cost: cost + edge.cost, node: edge.to };
                    if next.cost < dist[&next.node] {
                        dist.insert(next.node, next.cost);
                        heap.push(next);
                    }
                }
            }
        }

        None
    }
}

#[derive(PartialEq, Eq)]
pub struct PathFinderState {
    node: (i64, i64),
    cost: u64
}

impl Ord for PathFinderState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // Flip cost comparison
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for PathFinderState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
