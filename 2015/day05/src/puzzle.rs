use std::str::FromStr;

#[derive(Debug)]
pub struct PuzzleInput {
    strings: Vec<String>
}

impl PuzzleInput {
    pub fn new(strings: Vec<String>) -> PuzzleInput {
        PuzzleInput { strings }
    }

    pub fn part1_nice_strings<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        self.strings.iter().filter_map(|s| if part1::is_nice_string(&s) { Some(s.as_str()) } else { None })
    }

    pub fn part2_nice_strings<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        self.strings.iter().filter_map(|s| if part2::is_nice_string(&s) { Some(s.as_str()) } else { None })
    }
}

impl FromStr for PuzzleInput {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let strings = s.lines().map(|line| line.to_owned()).collect();
        Ok(PuzzleInput::new(strings))
    }
}

mod part1 {
    pub fn is_nice_string(s: &str) -> bool {
        has_three_vowels(s) &&
        has_doubled_letter(s) &&
        !contains_naughty_letter_pairs(s)
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
}

mod part2 {
    pub fn is_nice_string(s: &str) -> bool {
        has_nice_pair(s) && has_doubled_letter_with_gap_of_one(s)
    }

    fn has_nice_pair(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        for i in 0..(chars.len() - 3) {
            let a = chars[i];
            let b = chars[i + 1];

            let mut pair = a.to_string();
            pair.push(b);

            if s[(i + 2)..].contains(&pair) {
                return true;
            }
        }

        return false;
    }

    fn has_doubled_letter_with_gap_of_one(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        for i in 0..(chars.len() - 2) {
            let a = chars[i];
            let b = chars[i + 2];

            if a == b {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part1_examples() {
        assert!(part1::is_nice_string("ugknbfddgicrmopn"));
        assert!(part1::is_nice_string("aaa"));
        assert!(!part1::is_nice_string("jchzalrnumimnmhp"));
        assert!(!part1::is_nice_string("haegwjzuvuyypxyu"));
        assert!(!part1::is_nice_string("dvszwmarrgswjxmb"));
    }

    #[test]
    pub fn part2_examples() {
        assert!(part2::is_nice_string("qjhvhtzxzqqjkmpb"));
        assert!(part2::is_nice_string("xxyxx"));
        assert!(!part2::is_nice_string("uurcxstgmygtbstg"));
        assert!(!part2::is_nice_string("ieodomkazucvgmuy"));
    }
}
