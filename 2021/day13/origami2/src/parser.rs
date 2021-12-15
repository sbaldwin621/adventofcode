use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{line_ending, space0},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, pair}
};

use crate::{
    paper::Point,
    puzzle::{Instruction, PuzzleInput}
};

pub fn parse_puzzle_input(input: &str) -> IResult<&str, PuzzleInput> {
    map(
        separated_pair(point_set, pair(line_ending, line_ending), instruction_set),
        |(points, instructions)| PuzzleInput::new(points, instructions)
    )(input)
}

fn instruction_set(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((fold_up, fold_left))(input)
}

fn fold_up(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("fold along y="), number), |amount| Instruction::FoldUp(amount))(input)
}

fn fold_left(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("fold along x="), number), |amount| Instruction::FoldLeft(amount))(input)
}

fn point_set(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(line_ending, point)(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(number, tag(","), number),
        |(x, y)| Point(x, y)
    )(input)
}

fn number(input: &str) -> IResult<&str, i64> {
    delimited(space0, map_res(
        take_while(is_digit), 
        |s: &str| s.parse::<i64>()
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
            6,10
            0,14
            
            fold along y=7
            fold along x=5       
        ")).unwrap();
        
        assert_eq!(PuzzleInput::new(vec![
            Point(6, 10),
            Point(0, 14)
        ], vec![
            Instruction::FoldUp(7),
            Instruction::FoldLeft(5)
        ]), result);
    }
}