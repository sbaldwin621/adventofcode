use std::collections::HashSet;
use std::fmt::Display;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct Paper {
    dots: HashSet<Point>
}

impl Paper {
    pub fn new(dots: HashSet<Point>) -> Paper {
        Paper { dots }
    }

    pub fn fold_up(&mut self, fold_y: i64) {
        let mut next_dots = HashSet::new();

        for &Point(x, y) in self.dots.iter() {
            if y <= fold_y {
                next_dots.insert(Point(x, y));
            } else {
                let diff = y - fold_y;
                next_dots.insert(Point(x, fold_y - diff));
            }
        }

        self.dots = next_dots;
    }


    pub fn fold_left(&mut self, fold_x: i64) {
        let mut next_dots = HashSet::new();

        for &Point(x, y) in self.dots.iter() {
            if x <= fold_x {
                next_dots.insert(Point(x, y));
            } else {
                let diff = x - fold_x;
                next_dots.insert(Point(fold_x - diff, y));
            }
        }

        self.dots = next_dots;
    }

    pub fn len(&self) -> usize {
        self.dots.len()
    }

    pub fn dimensions(&self) -> (i64, i64) {
        let max_x = self.dots.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
        let max_y = self.dots.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

        (max_x, max_y)
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self.dimensions();

        for y in 0..=max_y {
            for x in 0..=max_x {
                write!(f, "{}", match self.dots.contains(&Point(x, y)) {
                    true => "#",
                    false => "."
                })?;
            }

            writeln!(f)?;
        }
        
        writeln!(f)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point(pub i64, pub i64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let dots = HashSet::from_iter([
            Point(6, 10),
            Point(0, 14),
            Point(9, 10),
            Point(0, 3),
            Point(10, 4),
            Point(4, 11),
            Point(6, 0),
            Point(6, 12),
            Point(4, 1),
            Point(0, 13),
            Point(10, 12),
            Point(3, 4),
            Point(3, 0),
            Point(8, 4),
            Point(1, 10),
            Point(2, 14),
            Point(8, 10),
            Point(9, 0)
        ]);

        let mut paper = Paper::new(dots);

        assert_eq!(18, paper.dots.len());

        paper.fold_up(7);

        assert_eq!(17, paper.dots.len());

        paper.fold_left(5);

        assert_eq!(16, paper.dots.len());
    }
}