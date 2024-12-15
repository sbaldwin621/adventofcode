use std::collections::HashMap;
use std::str::FromStr;

pub struct StoneLine {
    pub stones: Vec<u64>,
    known_results: HashMap<(u64, usize), Vec<u64>>
}

impl StoneLine {
    pub fn new(stones: Vec<u64>) -> StoneLine {
        let known_results = HashMap::new();

        StoneLine { stones, known_results }
    }

    pub fn len(&self) -> usize {
        self.stones.len()
    }

    pub fn simulate(&mut self, steps: usize) -> usize {
        let mut score = 0;

        for i in 0..self.stones.len() {
            let stone = self.stones[i];
            score += self.simulate_stone(stone, steps).len();
        }

        score
    }

    fn simulate_stone(&mut self, stone: u64, steps: usize) -> Vec<u64> {
        if let Some(results) = self.known_results.get(&(stone, steps)) {
            results.clone()
        } else if steps > 1 {
            let previous_step_results = self.simulate_stone(stone, steps - 1);
            let results = self.step_multiple(&previous_step_results);

            self.known_results.insert((stone, steps), results.clone());
            
            results
        } else {
            let results = StoneLine::step(stone);

            self.known_results.insert((stone, steps), results.clone());
            
            results
        }
    }

    fn step_multiple(&mut self, stones: &Vec<u64>) -> Vec<u64> {
        stones.iter().flat_map(|stone| self.simulate_stone(*stone, 1)).collect()
    }
    
    fn step(stone: u64) -> Vec<u64> {
        // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
        if stone == 0 {
            return vec![1];
        }

        // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
        let stone_string = stone.to_string();
        let len = stone_string.len();
        if len % 2 == 0 {
            let (left, right) = stone_string.split_at(len / 2);

            let left: u64 = left.parse().unwrap();
            let right: u64 = right.parse().unwrap();

            return vec![left, right];
        }

        // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
        vec![stone * 2024]
    }
}

impl FromStr for StoneLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();

        Ok(StoneLine::new(stones))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn blink() {
        let mut line: StoneLine = "0 1 10 99 999".parse().unwrap();

        // line.blink();

        assert_eq!(line.stones, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }
}