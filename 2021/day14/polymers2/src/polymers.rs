use std::collections::HashMap;
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
    counts: HashMap<Polymer, u64>,
    pairs: HashMap<(Polymer, Polymer), u64>
}

impl PolymerChain {
    pub fn new(polymers: Vec<Polymer>) -> PolymerChain {
        let mut counts = HashMap::new();
        let mut pairs = HashMap::new();
        
        for i in 0..polymers.len() - 1 {
            let first = &polymers[i];
            let second = &polymers[i + 1];

            let pair = (first.clone(), second.clone());
            
            *pairs.entry(pair).or_insert(0) += 1;

            *counts.entry(first.clone()).or_insert(0) += 1;
        }

        // Add the last one
        *counts.entry(polymers[polymers.len() - 1].clone()).or_insert(0) += 1;

        PolymerChain { counts, pairs }
    }

    pub fn apply_rules(&mut self, rules: &Vec<InsertionRule>) {
        let mut new_pairs = HashMap::new();

        for rule in rules.iter() {
            if let Some(&count) = self.pairs.get(&rule.target) {
                let insert = &rule.insert;
                let (first, second) = &rule.target;

                let first_pair = (first.clone(), insert.clone());
                let second_pair = (insert.clone(), second.clone());

                *new_pairs.entry(first_pair.clone()).or_insert(0) += count;
                *new_pairs.entry(second_pair.clone()).or_insert(0) += count;

                *self.counts.entry(insert.clone()).or_insert(0) += count;                
            }
        }

        self.pairs = new_pairs;
    }
    
    pub fn score(&self) -> u64 {
        let (_, &minimum) = self.counts.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let (_, &maximum) = self.counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

        maximum - minimum
    }

    // pub fn apply_rules(&mut self, rules: &Vec<InsertionRule>) {
    //     let mut next_polymers = vec![];

    //     for i in 0..self.polymers.len() - 1 {
    //         let a = &self.polymers[i];
    //         let b = &self.polymers[i + 1];

    //         next_polymers.push(a.clone());

    //         for rule in rules.iter() {
    //             if rule.first == *a && rule.second == *b {
    //                 next_polymers.push(rule.result.clone());
    //             }
    //         }
    //     }

    //     let last = &self.polymers[self.polymers.len() - 1];
    //     next_polymers.push(last.clone());

    //     self.polymers = next_polymers;
    // }

    // pub fn score(&self) -> usize {
    //     let counts = self.polymers.iter().counts_by(|polymer| &polymer.0);
    //     let (_, &minimum) = counts.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    //     let (_, &maximum) = counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    //     maximum - minimum
    // }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Polymer(pub String);

impl Polymer {
    pub fn from_str(s: &str) -> Polymer {
        Polymer(s.to_string())
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct InsertionRule {
    target: (Polymer, Polymer),
    insert: Polymer
}

impl InsertionRule {
    pub fn new(first: Polymer, second: Polymer, insert: Polymer) -> InsertionRule {
        let target = (first.clone(), second.clone());
        
        InsertionRule { target, insert }
    }

    pub fn from_str(first: &str, second: &str, insert: &str) -> InsertionRule {
        InsertionRule::new(
            Polymer::from_str(first),
            Polymer::from_str(second),
            Polymer::from_str(insert)
        )
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