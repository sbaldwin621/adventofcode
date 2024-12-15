use std::str::FromStr;

pub struct StoneLine {
    stones: Vec<u64>
}

impl StoneLine {
    pub fn new(stones: Vec<u64>) -> StoneLine {
        StoneLine { stones }
    }

    pub fn len(&self) -> usize {
        self.stones.len()
    }

    pub fn blink(&mut self) {
        let mut next_stones = vec![];

        for stone in self.stones.iter() {
            let stone = *stone;

            // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
            if stone == 0 {
                next_stones.push(1);

                continue;
            }

            // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
            let stone_string = stone.to_string();
            let len = stone_string.len();
            if len % 2 == 0 {
                let (left, right) = stone_string.split_at(len / 2);

                let left: u64 = left.parse().unwrap();
                let right: u64 = right.parse().unwrap();

                next_stones.push(left);
                next_stones.push(right);

                continue;
            }

            // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
            next_stones.push(stone * 2024);   
        }
        
        self.stones = next_stones;
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

        line.blink();
        
        assert_eq!(line.stones, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }
}