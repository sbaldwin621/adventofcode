#[derive(Debug, PartialEq, Eq)]
pub struct PuzzleInput {
    line_segments: Vec<LineSegment>
}

impl PuzzleInput {
    pub fn new(line_segments: Vec<LineSegment>) -> PuzzleInput {
        PuzzleInput { line_segments }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LineSegment {
    one: Point,
    two: Point
}

impl LineSegment {
    pub fn new(one: Point, two: Point) -> LineSegment {
        LineSegment { one, two }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: i64,
    y: i64
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}