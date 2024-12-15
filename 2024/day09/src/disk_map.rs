use std::str::FromStr;

use thiserror::Error;

pub struct DiskMap {
    blocks: Vec<Option<usize>>
}

impl DiskMap {
    pub fn new(blocks: Vec<Option<usize>>) -> DiskMap {
        DiskMap { blocks }
    }
}

impl FromStr for DiskMap {
    type Err = DiskMapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = vec![];
        let mut next_id = 0;

        let chars: Vec<char> = s.chars().collect();

        for i in (0..chars.len()).step_by(2) {
            if let Some(char) = chars.get(i) {
                if let Some(block_count) = char.to_digit(10) {
                    let block_count = block_count as usize;

                    let id = next_id;
                    next_id += 1;

                    for _ in 0..block_count {
                        blocks.push(Some(id));
                    }
                }
            }

            if let Some(char) = chars.get(i + 1) {
                if let Some(block_count) = char.to_digit(10) {
                    for _ in 0..block_count {
                        blocks.push(None);
                    }
                }
            }
        }

        println!("{:?}", blocks);

        Ok(DiskMap::new(blocks))
    }
}

#[derive(Debug, Error)]
pub enum DiskMapParseError { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parses() {
        let disk_map: DiskMap = "2333133121414131402".parse().unwrap();

        assert_eq!(disk_map.blocks, vec![
            Some(0), Some(0),
            None, None, None,
            Some(1), Some(1), Some(1),
            None, None, None,
            Some(2),
            None, None, None,
            Some(3), Some(3), Some(3),
            None,
            Some(4), Some(4),
            None,
            Some(5), Some(5), Some(5), Some(5),
            None,
            Some(6), Some(6), Some(6), Some(6), 
            None,
            Some(7), Some(7), Some(7),
            None,
            Some(8), Some(8), Some(8), Some(8),
            Some(9), Some(9)    
        ]);
    }
}