use nom::IResult;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{space0, line_ending};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, delimited};

use crate::puzzle::{Vector2, PuzzleInput, LineSegment};

pub fn parse_puzzle_input(input: &str) -> IResult<&str, PuzzleInput> {
    map(
        separated_list1(line_ending, line_segment),
        |line_segments| PuzzleInput::new(line_segments)
    )(input)
}

pub fn line_segment(input: &str) -> IResult<&str, LineSegment> {
    map(
        separated_pair(point, tag("->"), point),
        |(one, two)| LineSegment::new(one, two)
    )(input)
}

pub fn point(input: &str) -> IResult<&str, Vector2> {
    delimited(
        space0, 
        map(
            separated_pair(number, tag(","), number),
            |(x, y)| Vector2::new(x, y)
        ),
        space0
    )(input)
}

fn number(input: &str) -> IResult<&str, f64> {
    delimited(space0, map_res(
        take_while(is_digit), 
        |s: &str| s.parse::<f64>()
    ), space0)(input)
}

fn is_digit(chr: char) -> bool {
    chr.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn parse_example() {
        let (_, result) = parse_puzzle_input(indoc!("
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2       
        ")).unwrap();

        assert_eq!(PuzzleInput::new(vec![
            LineSegment::new(Vector2::new(0.0, 9.0), Vector2::new(5.0, 9.0)),
            LineSegment::new(Vector2::new(8.0, 0.0), Vector2::new(0.0, 8.0)),
            LineSegment::new(Vector2::new(9.0, 4.0), Vector2::new(3.0, 4.0)),
            LineSegment::new(Vector2::new(2.0, 2.0), Vector2::new(2.0, 1.0)),
            LineSegment::new(Vector2::new(7.0, 0.0), Vector2::new(7.0, 4.0)),
            LineSegment::new(Vector2::new(6.0, 4.0), Vector2::new(2.0, 0.0)),
            LineSegment::new(Vector2::new(0.0, 9.0), Vector2::new(2.0, 9.0)),
            LineSegment::new(Vector2::new(3.0, 4.0), Vector2::new(1.0, 4.0)),
            LineSegment::new(Vector2::new(0.0, 0.0), Vector2::new(8.0, 8.0)),
            LineSegment::new(Vector2::new(5.0, 5.0), Vector2::new(8.0, 2.0))
        ]), result);
    }
}