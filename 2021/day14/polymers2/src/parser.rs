use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{line_ending, space0},
    combinator::{map},
    multi::{separated_list1, many1},
    sequence::{delimited, separated_pair, pair}
};

use crate::{
    polymers::{PuzzleInput, Polymer, InsertionRule, PolymerChain}
};

pub fn parse_puzzle_input(input: &str) -> IResult<&str, PuzzleInput> {
    map(
        separated_pair(polymer_chain, pair(line_ending, line_ending), insertion_rule_set),
        |(polymers, rules)| PuzzleInput::new(polymers, rules)
    )(input)
}

fn insertion_rule_set(input: &str) -> IResult<&str, Vec<InsertionRule>> {
    separated_list1(line_ending, insertion_rule)(input)
}

fn insertion_rule(input: &str) -> IResult<&str, InsertionRule> {
    map(
        separated_pair(polymer_pair, tag("->"), polymer),
        |((first, second), result)| InsertionRule::new(first, second, result)
    )(input)
}

fn polymer_chain(input: &str) -> IResult<&str, PolymerChain> {
    map(
        delimited(space0, many1(polymer), space0),
        |polymers| PolymerChain::new(polymers)
    )(input)
}

fn polymer_pair(input: &str) -> IResult<&str, (Polymer, Polymer)> {
    delimited(space0, pair(polymer, polymer), space0)(input)
}

fn polymer(input: &str) -> IResult<&str, Polymer> {
    delimited(space0, map(
        take_while_m_n(1, 1, is_letter),
        |s: &str| Polymer(s.to_string())
    ), space0)(input)
}

fn is_letter(chr: char) -> bool {
    chr.is_alphabetic()
}

#[cfg(test)]
mod tests {
    use std::net::ToSocketAddrs;

    use indoc::indoc;

    use crate::polymers::PolymerChain;

    use super::*;

    #[test]
    fn parse_example() {
        let (_, result) = parse_puzzle_input(indoc!("
            NNCB

            CH -> B
            HH -> N
            CB -> H
            NH -> C
            HB -> C
            HC -> B
            HN -> C
            NN -> C
            BH -> H
            NC -> B
            NB -> B
            BN -> B
            BB -> N
            BC -> B
            CC -> N
            CN -> C     
        ")).unwrap();
        
        assert_eq!(PuzzleInput::new(PolymerChain::new(vec![
            Polymer::from_str("N"),
            Polymer::from_str("N"),
            Polymer::from_str("C"),
            Polymer::from_str("B")
        ]), vec![
            InsertionRule::from_str("C", "H", "B"),
            InsertionRule::from_str("H", "H", "N"),
            InsertionRule::from_str("C", "B", "H"),
            InsertionRule::from_str("N", "H", "C"),
            InsertionRule::from_str("H", "B", "C"),
            InsertionRule::from_str("H", "C", "B"),
            InsertionRule::from_str("H", "N", "C"),
            InsertionRule::from_str("N", "N", "C"),
            InsertionRule::from_str("B", "H", "H"),
            InsertionRule::from_str("N", "C", "B"),
            InsertionRule::from_str("N", "B", "B"),
            InsertionRule::from_str("B", "N", "B"),
            InsertionRule::from_str("B", "B", "N"),
            InsertionRule::from_str("B", "C", "B"),
            InsertionRule::from_str("C", "C", "N"),
            InsertionRule::from_str("C", "N", "C"),
        ]), result);
    }
}