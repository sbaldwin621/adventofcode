use std::collections::HashMap;
use std::str::FromStr;

pub struct StoneLine {
    pub stones: HashMap<u64, usize>
}

impl StoneLine {
    pub fn new(stones: HashMap<u64, usize>) -> StoneLine {
        StoneLine { stones }
    }

    pub fn score(&self) -> usize {
        self.stones.iter().map(|(_, count)| *count).sum()
    }

    pub fn blink(&mut self) {
        let mut next_stones = HashMap::new();

        for (stone, count) in self.stones.iter() {
            let stone = *stone;
            let count = *count;

            for result in StoneLine::step(stone) {
                next_stones.entry(result)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }

        self.stones = next_stones;
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
        let mut stones = HashMap::new();

        for segment in s.split_whitespace() {
            let stone = segment.parse::<u64>().unwrap();

            stones.entry(stone)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        Ok(StoneLine::new(stones))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // pub fn blink() {
    //     let mut line: StoneLine = "0 1 10 99 999".parse().unwrap();

    //     // line.blink();

    //     assert_eq!(line.stones, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    // }
}