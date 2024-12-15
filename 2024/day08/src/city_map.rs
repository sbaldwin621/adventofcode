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

    pub fn count_antinodes_within_map_part1(&self) -> usize {
        let antinodes = self.get_antinodes_part1();

        antinodes.len()
    }

    pub fn count_antinodes_within_map_part2(&self) -> usize {
        let antinodes = self.get_antinodes_part2();

        antinodes.len()
    }

    fn get_antinodes_part1(&self) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        for (_, points) in self.antennas_by_symbol.iter() {
            for (i, point) in points.iter().enumerate() {
                for other in points.iter().skip(i + 1) {
                    let Point(x, y) = other.subtract(point);
                    
                    let before_antinode = Point(point.0 - x, point.1 - y);
                    if self.is_point_in_map(before_antinode) {
                        antinodes.insert(before_antinode);
                    }
                    
                    let after_antinode = Point(other.0 + x, other.1 + y);
                    if self.is_point_in_map(after_antinode) {
                        antinodes.insert(after_antinode);
                    }
                }
            }
        }

        antinodes
    }

    fn get_antinodes_part2(&self) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        for (_, points) in self.antennas_by_symbol.iter() {
            for (i, point) in points.iter().enumerate() {
                for other in points.iter().skip(i + 1) {
                    let Point(x, y) = other.subtract(point);
                    
                    antinodes.insert(*point);
                    antinodes.insert(*other);

                    for n in 1..=usize::MAX {
                        let n: i32 = n.try_into().unwrap();

                        let antinode = Point(point.0 - x * n, point.1 - y * n);
                        if self.is_point_in_map(antinode) {
                            antinodes.insert(antinode);
                        } else {
                            break;
                        }
                    }

                    for n in 1..=usize::MAX {
                        let n: i32 = n.try_into().unwrap();

                        let antinode = Point(other.0 + x * n, other.1 + y * n);
                        if self.is_point_in_map(antinode) {
                            antinodes.insert(antinode);
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        antinodes
    }

    fn is_point_in_map(&self, point: Point) -> bool {
        let width: i32 = self.width.try_into().unwrap();
        let height: i32 = self.height.try_into().unwrap();

        let Point(x, y) = point;

        x >= 0 && x < width && y >= 0 && y < height
    }
}