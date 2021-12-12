use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct MapBuilder {
    map: Vec<u8>,
    width: Option<usize>
}

impl MapBuilder {
    pub fn new() -> MapBuilder {
        MapBuilder { width: None, map: vec![] }
    }

    pub fn add_line(&mut self, line: &String) {
        if self.width == None {
            self.width = Some(line.len());
        }

        for char in line.chars() {
            self.map.push(char.to_string().parse::<u8>().unwrap());
        }
    }

    pub fn to_map(self) -> Map {
        let map = self.map;
        let width = self.width.unwrap();
        let height = map.len() / width;

        Map { map, width, height }
    }
}

#[derive(Debug)]
pub struct Map {
    map: Vec<u8>,
    height: usize,
    width: usize
}

impl Map {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: i64, y: i64) -> Option<u8> {
        let i: i64 = y * (self.width as i64) + x;
        if i >= 0 {
            self.map.get(i as usize).map(|v| *v)
        } else {
            None
        }
    }

    pub fn is_low_point(&self, x: i64, y: i64) -> bool {
        let this = self.get(x, y).unwrap();
        let north = self.get(x, y - 1);
        let west = self.get(x - 1, y);
        let east = self.get(x + 1, y);
        let south = self.get(x, y + 1);

        if let Some(north) = north {
            if north <= this {
                return false;
            }
        }

        if let Some(east) = east {
            if east <= this {
                return false;
            }
        }
        
        if let Some(south) = south {
            if south <= this {
                return false;
            }
        }
        
        if let Some(west) = west {
            if west <= this {
                return false;
            }
        }

        return true;
    }

    pub fn calculate_risk_level(&self) -> u64 {
        let mut total_risk_level: u64 = 0;

        for i in 0..self.map.len() {
            let x = (i % self.width) as i64;
            let y = (i / self.width) as i64;

            if self.is_low_point(x, y) {
                total_risk_level += (*self.map.get(i).unwrap()) as u64 + 1;
            }
        }

        total_risk_level
    }

    pub fn calculate_basin_score(&self) -> u64 {
        let mut basin_sizes = vec![];

        for i in 0..self.map.len() {
            let x = (i % self.width) as i64;
            let y = (i / self.width) as i64;

            if self.is_low_point(x, y) {
                basin_sizes.push(self.get_basin_size(x, y));
            }
        }

        basin_sizes.sort();

        basin_sizes.iter().rev().take(3).fold(1, |a, b| a * b)
    }

    fn get_basin_size(&self, x: i64, y: i64) -> u64 {
        let mut points_in_basin = HashSet::new();

        self.trace_basin(x, y, &mut points_in_basin);

        points_in_basin.len() as u64
    }

    fn trace_basin(&self, x: i64, y: i64, points_in_basin: &mut HashSet<(i64, i64)>) {
        if x < 0 || x >= (self.width as i64) || y < 0 || y >= (self.height as i64) || points_in_basin.contains(&(x, y)) {
            return;
        }

        if let Some(value) = self.get(x, y) {
            if value < 9 {
                points_in_basin.insert((x, y));

                self.trace_basin(x - 1, y, points_in_basin);
                self.trace_basin(x + 1, y, points_in_basin);
                self.trace_basin(x, y - 1, points_in_basin);
                self.trace_basin(x, y + 1, points_in_basin);
            }
        };
    }    
}