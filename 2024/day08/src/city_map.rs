use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point(i32, i32);

impl Point {
    pub fn subtract(&self, other: &Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

pub struct CityMapBuilder {
    next_row: usize,
    width: usize,
    antennas_by_symbol: HashMap<char, Vec<Point>>
}

impl CityMapBuilder {
    pub fn new() -> CityMapBuilder {
        let next_row = 0;
        let width = 0;
        let antennas_by_symbol = HashMap::new();
        
        CityMapBuilder { next_row, width, antennas_by_symbol }
    }

    pub fn add_line(&mut self, line: &String) {
        self.width = line.len();

        let row = i32::try_from(self.next_row).unwrap();
        self.next_row += 1;

        for (col, char) in line.chars().enumerate() {
            let col = i32::try_from(col).unwrap();
            
            if char == '.' || char == '#' {
                continue;
            }
            
            let entry = self.antennas_by_symbol.entry(char).or_insert_with(|| vec![]);
            entry.push(Point(col, row))
        }
    }

    pub fn into_city_map(self) -> CityMap {
        let height = self.next_row;

        CityMap::new(self.width, height, self.antennas_by_symbol)
    }
}

pub struct CityMap {
    width: usize,
    height: usize,
    antennas_by_symbol: HashMap<char, Vec<Point>>
}

impl CityMap {
    pub fn new(width: usize, height: usize, antennas_by_symbol: HashMap<char, Vec<Point>>) -> CityMap {
        CityMap { width, height, antennas_by_symbol }
    }

    pub fn count_antinodes_within_map(&self) -> u32 {
        let antinodes = self.get_antinodes();

        let mut score = 0;

        let width: i32 = self.width.try_into().unwrap();
        let height: i32 = self.height.try_into().unwrap();

        for Point(x, y) in antinodes.iter() {
            let x = *x;
            let y = *y;

            if x >= 0 && x < width && y >= 0 && y < height {
                score += 1;
            }
        }

        score
    }


    fn get_antinodes(&self) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        for (symbol, points) in self.antennas_by_symbol.iter() {
            for (i, point) in points.iter().enumerate() {
                for other in points.iter().skip(i + 1) {
                    let Point(x, y) = other.subtract(point);

                    let antinode_one = Point(point.0 - x, point.1 - y);
                    let antinode_two = Point(other.0 + x, other.1 + y);
                    
                    antinodes.insert(antinode_one);
                    antinodes.insert(antinode_two);

                    // println!("{:?} {:?}+{:?}: {:?}, {:?}", symbol, point, other, antinode_one, antinode_two)
                }
            }
        }

        antinodes
    }
}