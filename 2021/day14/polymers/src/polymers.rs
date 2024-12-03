use std::fmt::Display;

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug)]
pub struct PuzzleInput {
    pub polymer_template: PolymerChain,
    pub insertion_rules: Vec<InsertionRule>
}

impl PuzzleInput {
    pub fn new(polymer_template: PolymerChain, insertion_rules: Vec<InsertionRule>) -> PuzzleInput {
        PuzzleInput { polymer_template, insertion_rules }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PolymerChain {
    polymers: Vec<Polymer>
}

impl PolymerChain {
    pub fn new(polymers: Vec<Polymer>) -> PolymerChain {
        PolymerChain { polymers }
    }

    pub fn apply_rules(&mut self, rules: &Vec<InsertionRule>) {
        let mut next_polymers = vec![];

        for i in 0..self.polymers.len() - 1 {
            let a = &self.polymers[i];
            let b = &self.polymers[i + 1];

            next_polymers.push(a.clone());

            for rule in rules.iter() {
                if rule.first == *a && rule.second == *b {
                    next_polymers.push(rule.result.clone());
                }
            }
        }

        let last = &self.polymers[self.polymers.len() - 1];
        next_polymers.push(last.clone());

        self.polymers = next_polymers;
    }

    pub fn counts(&self) -> std::collections::HashMap<&String, usize> {
        self.polymers.iter().counts_by(|polymer| &polymer.0)
    }

    pub fn score(&self) -> usize {
        let counts = self.polymers.iter().counts_by(|polymer| &polymer.0);
        let (_, &minimum) = counts.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let (_, &maximum) = counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

        maximum - minimum
    }
}

impl Display for PolymerChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for polymer in self.polymers.iter() {
            write!(f, "{}", polymer.0)?
        })
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Polymer(pub String);

impl Polymer {
    pub fn from_str(s: &str) -> Polymer {
        Polymer(s.to_string())
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct InsertionRule {
    first: Polymer,
    second: Polymer,
    result: Polymer
}

impl InsertionRule {
    pub fn new(first: Polymer, second: Polymer, result: Polymer) -> InsertionRule {
        InsertionRule { first, second, result }
    }

    pub fn from_str(first: &str, second: &str, result: &str) -> InsertionRule {
        InsertionRule::new(
            Polymer::from_str(first),
            Polymer::from_str(second),
            Polymer::from_str(result))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut chain = PolymerChain::new(vec![
            Polymer::from_str("A"),
            Polymer::from_str("B"),
            Polymer::from_str("C"),
        ]);
        
        chain.apply_rules(&vec![
            InsertionRule::from_str("A", "B", "C"),
            InsertionRule::from_str("B", "C", "D"),
        ]);

        let expected = PolymerChain::new(vec![
            Polymer::from_str("A"),
            Polymer::from_str("C"),
            Polymer::from_str("B"),
            Polymer::from_str("D"),
            Polymer::from_str("C")
        ]);

        assert_eq!(expected, chain);
    }
}