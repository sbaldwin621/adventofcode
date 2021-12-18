use std::collections::{HashMap, HashSet, BinaryHeap};

#[derive(Debug)]
pub struct MapBuilder {
    values: Vec<u64>,
    width: Option<usize>,
    height: usize
}

impl MapBuilder {
    pub fn new() -> MapBuilder {
        let lines = vec![];
        let width = None;

        MapBuilder { values: lines, width, height: 0 }
    }

    pub fn add_line(&mut self, line: String) {
        if self.width == None {
            self.width = Some(line.len());
        }

        for value in line.chars().map(|c| c.to_string().parse::<u64>().unwrap()) {
            self.values.push(value);
        }

        self.height += 1;
    }

    pub fn to_map(&self) -> Map {
        let width: i64 = self.width.unwrap().try_into().unwrap();
        let height: i64 = self.height.try_into().unwrap();

        let mut nodes = HashMap::new();
        
        for x in 0..width {
            for y in 0..height {
                if let Some(cost) = self.get((x, y)) {
                    for x_n in 0..5 {
                        for y_n in 0..5 {
                            let x = x + width * x_n;
                            let y = y + height * y_n;
                            
                            let cost = (cost + (x_n as u64) + (y_n as u64)) % 9;
                            let cost = if cost == 0 { 9 } else { cost };
                            
                            nodes.insert((x, y), cost);
                        }
                    }
                }
            }
        }

        Map { nodes, width: width * 5, height: height * 5 }
    }

    fn get(&self, pos: (i64, i64)) -> Option<u64> {
        let width: i64 = self.width.unwrap().try_into().unwrap();
        let (x, y) = pos;
        if x < 0 || y < 0 || x >= width {
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
pub struct Map {
    nodes: HashMap<(i64, i64), u64>,
    width: i64,
    height: i64
}

impl Map {
    pub fn corner(&self) -> (i64, i64) {
        let x = self.width - 1;
        let y = self.height - 1;

        (x, y)
    }

    pub fn shortest_path(&self, start: (i64, i64), goal: (i64, i64)) -> Option<u64> {
        let mut dist = HashMap::new();
        for (&node, _) in self.nodes.iter() {
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

            let (x, y) = node;
            let north = (x, y - 1);
            let east = (x + 1, y);
            let south = (x, y + 1);
            let west = (x - 1, y);

            for next in [north, east, south, west] {
                if let Some(next_cost) = self.nodes.get(&next) {
                    let state = PathFinderState { cost: cost + next_cost, node: next };
                    if state.cost < dist[&next] {
                        dist.insert(next, state.cost);
                        heap.push(state);
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
