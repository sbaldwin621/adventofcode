use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::preceded;

use crate::puzzle::{Instruction, PuzzleInput};

pub fn parse_puzzle_input(input: &str) -> IResult<&str, PuzzleInput> {
    map(
        separated_list1(line_ending, instruction),
        |instruction_sets| PuzzleInput::new(instruction_sets)
    )(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((forward, down, up))(input)
}

fn forward(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("forward"), number), |amount| Instruction::Forward(amount))(input)
}

fn down(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("down"), number), |amount| Instruction::Down(amount))(input)
}

fn up(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("up"), number), |amount| Instruction::Up(amount))(input)
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
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2        
        ")).unwrap();
        
        assert_eq!(PuzzleInput::new(vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2)
        ]), result);
    }
}