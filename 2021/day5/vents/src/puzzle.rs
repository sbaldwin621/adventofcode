use std::collections::HashMap;
use std::hash::Hash;

use crate::parser::line_segment;

#[derive(Debug, PartialEq)]
pub struct PuzzleInput {
    line_segments: Vec<LineSegment>
}

impl PuzzleInput {
    pub fn new(line_segments: Vec<LineSegment>) -> PuzzleInput {
        PuzzleInput { line_segments }
    }

    pub fn line_segments(&self) -> std::slice::Iter<LineSegment> {
        self.line_segments.iter()
    }
}

#[derive(Debug, PartialEq)]
pub struct LineSegment {
    one: Vector2,
    two: Vector2
}

impl LineSegment {
    pub fn new(one: Vector2, two: Vector2) -> LineSegment {
        LineSegment { one, two }
    }

    pub fn is_horizontal(&self) -> bool {
        self.one.y == self.two.y
    }

    pub fn is_vertical(&self) -> bool {
        self.one.x == self.two.x
    }

    pub fn vector(&self) -> Vector2 {
        Vector2::new(self.two.x - self.one.x, self.two.y - self.one.y)
    }
}

#[derive(Debug, PartialEq)]
pub struct Vector2 {
    x: f64,
    y: f64
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn unit(&self) -> Vector2 {
        let length = self.length();
        Vector2::new(self.x / length, self.y / length)
    }

    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }

    pub fn multiply(&self, value: f64) -> Vector2 {
        Vector2::new(self.x * value, self.y * value)
    }
}

#[derive(Debug)]
pub struct Solver {
    overlap_map: HashMap<(i64, i64), u64>
}

impl Solver {
    pub fn new() -> Solver {
        Solver { overlap_map: HashMap::new() }
    }
    
    pub fn ingest(&mut self, line_segment: &LineSegment) {
        if line_segment.is_horizontal() || line_segment.is_vertical() {
            let vector = line_segment.vector();
            let length = vector.length().round() as u64;
            let unit = vector.unit();

            let start = &line_segment.one;
            for n in 0..length + 1 {
                let point = start.add(&unit.multiply(n as f64));
                let snapped_point = (point.x.round() as i64, point.y.round() as i64);

                let count = self.overlap_map.get(&snapped_point).unwrap_or(&0) + 1;
                self.overlap_map.insert(snapped_point, count);
            }
        }
    }

    pub fn count_overlaps(&self) -> u64 {
        let mut overlaps = 0;

        for (_, &count) in self.overlap_map.iter() {
            if count > 1 {
                overlaps += 1;
            }
        }

        overlaps
    }
}