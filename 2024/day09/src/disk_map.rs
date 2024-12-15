use std::collections::VecDeque;
use std::str::FromStr;

use thiserror::Error;

pub struct DiskMap {
    blocks: Vec<Option<usize>>
}

impl DiskMap {
    pub fn new(blocks: Vec<Option<usize>>) -> DiskMap {
        DiskMap { blocks }
    }

    pub fn compact(&mut self) {
        let mut empty_blocks = VecDeque::new();
        for (i, block) in self.blocks.iter().enumerate() {
            if let None = block {
                empty_blocks.push_back(i);
            }
        }

        for i in (0..self.blocks.len()).rev() {
            let block = self.blocks[i];
            if let Some(file_id) = block {
                if let Some(j) = empty_blocks.pop_front() {
                    // Never move files to the right
                    if j > i {
                        break;
                    }

                    self.blocks[j] = Some(file_id);
                    self.blocks[i] = None;
                }

                if empty_blocks.len() == 0 {
                    break;
                }
            }
        }
    }

    pub fn compact_without_fragmenting(&mut self) {
        let mut i = self.blocks.len() - 1;
        while i > 0 {
            let block = self.blocks[i];
            if let Some(file_id) = block {
                let mut start_of_block = i;
                for n in (0..i).rev() {
                    if self.blocks[n] != block {
                        start_of_block = n + 1;
                        break;
                    }
                }
                
                let length = i - start_of_block + 1;
                
                if let Some(j) = self.find_empty_space(length) {
                    // Only move files to the left
                    if j < i {
                        for n in 0..length {
                            self.blocks[j + n] = Some(file_id);
                            self.blocks[i - n] = None;
                        }
                    }
                }

                i = start_of_block - 1;
            } else {
                i -= 1;
            }
        }
    }

    fn find_empty_space(&self, length: usize) -> Option<usize> {
        let mut i = 0;
        loop {
            if i + length >= self.blocks.len() {
                return None;
            }

            if self.blocks.iter().skip(i).take(length).all(|block| block.is_none()) {
                return Some(i);
            }

            i += 1;
        }
    }

    pub fn checksum(&self) -> usize {
        let mut checksum = 0;

        for (i, block) in self.blocks.iter().enumerate() {
            if let Some(file_id) = block {
                let file_id = *file_id;

                checksum += file_id * i;
            }
        }

        checksum
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

    #[test]
    pub fn compacts() {
        let mut disk_map: DiskMap = "2333133121414131402".parse().unwrap();

        disk_map.compact();

        // 0099811188827773336446555566..............
        assert_eq!(disk_map.blocks, vec![
            Some(0), Some(0),
            Some(9), Some(9),
            Some(8),
            Some(1), Some(1), Some(1),
            Some(8), Some(8), Some(8),
            Some(2),
            Some(7), Some(7), Some(7), 
            Some(3), Some(3), Some(3),
            Some(6),
            Some(4), Some(4),
            Some(6), 
            Some(5), Some(5), Some(5), Some(5),
            Some(6), Some(6),
            None, None, None, None, None, None, None, None, None, None, None, None, None, None
        ]);
    }

    #[test]
    pub fn compacts_without_fragmenting() {
        let mut disk_map: DiskMap = "2333133121414131402".parse().unwrap();

        disk_map.compact_without_fragmenting();

        // 00992111777.44.333....5555.6666.....8888..
        assert_eq!(disk_map.blocks, vec![
            Some(0), Some(0),
            Some(9), Some(9),
            Some(2),
            Some(1), Some(1), Some(1),
            Some(7), Some(7), Some(7),
            None,
            Some(4), Some(4),
            None,
            Some(3), Some(3), Some(3),
            None, None, None, None,
            Some(5), Some(5), Some(5), Some(5),
            None,
            Some(6), Some(6), Some(6), Some(6),
            None, None, None, None, None,
            Some(8), Some(8), Some(8), Some(8),
            None, None
        ]);
    }


    #[test]
    pub fn checksum() {
        let mut disk_map: DiskMap = "2333133121414131402".parse().unwrap();

        disk_map.compact();

        assert_eq!(disk_map.checksum(), 1928);
    }
}