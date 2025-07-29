use std::str::FromStr;

use thiserror::Error;

pub fn is_nice_string(s: &str) -> bool {
    has_three_vowels(s) && has_doubled_letter(s) && !contains_naughty_letter_pairs(s)
}

fn has_three_vowels(s: &str) -> bool {
    s.chars().filter(|c| is_vowel(*c)).count() >= 3
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false
    }
}

fn has_doubled_letter(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..(chars.len() - 1) {
        let a = chars[i];
        let b = chars[i + 1];

        if a == b {
            return true;
        }
    }

    false
}

fn contains_naughty_letter_pairs(s: &str) -> bool {
    s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
}

#[derive(Debug)]
pub struct PuzzleInput {
    strings: Vec<String>
}

impl PuzzleInput {
    pub fn new(strings: Vec<String>) -> PuzzleInput {
        PuzzleInput { strings }
    }

    pub fn nice_strings<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        self.strings.iter().filter_map(|s| if is_nice_string(&s) { Some(s.as_str()) } else { None })
    }
}

impl FromStr for PuzzleInput {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let strings = s.lines().map(|line| line.to_owned()).collect();
        Ok(PuzzleInput::new(strings))
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::is_nice_string;

    use super::*;

    #[test]
    pub fn part1_examples() {
        assert!(is_nice_string("ugknbfddgicrmopn"));
        assert!(is_nice_string("aaa"));
        assert!(!is_nice_string("jchzalrnumimnmhp"));
        assert!(!is_nice_string("haegwjzuvuyypxyu"));
        assert!(!is_nice_string("dvszwmarrgswjxmb"));
    }
}
