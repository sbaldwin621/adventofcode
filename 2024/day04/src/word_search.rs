use std::ops::Index;
use std::str::Bytes;

pub struct WordSearch {
    lines: Vec<String>
}

impl WordSearch {
    pub fn new(lines: Vec<String>) -> WordSearch {
        WordSearch { lines }
    }

    pub fn search(&self, target: String) -> usize {
        let mut count = 0;

        let target_bytes = target.as_bytes();

        count += self.search_rows(&target);
        count += self.search_columns(&target_bytes);

        count
    }

    fn search_rows(&self, target: &String) -> usize {
        let target_reversed: String = target.chars().rev().collect();

        let mut count = 0;

        for line in self.lines.iter() {
            count += line.match_indices(target).count();
            count += line.match_indices(&target_reversed).count();
        }

        count
    }

    fn search_columns(&self, target: &[u8]) -> usize {
        // let target_reversed: Vec<u8> = target.iter().copied().rev().collect();
        // let target_reversed = std::str::from_utf8(&target_reversed).unwrap();
        
        // let target = std::str::from_utf8(&target).unwrap();
        

        let line_length = self.lines.first().unwrap().len();

        let mut count = 0;
        let mut target_i: usize = 0;

        let c = target[target_i];

        for x in 0..line_length {
            for y in 0..self.lines.len() {
                
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let word_search = WordSearch::new(vec![
            String::from("MMMSXXMASM"),
            String::from("MSAMXMSMSA"),
            String::from("AMXSXMAAMM"),
            String::from("MSAMASMSMX"),
            String::from("XMASAMXAMM"),
            String::from("XXAMMXXAMA"),
            String::from("SMSMSASXSS"),
            String::from("SAXAMASAAA"),
            String::from("MAMMMXMMMM"),
            String::from("MXMXAXMASX")
        ]);
        let count = word_search.search(String::from("XMAS"));

        assert_eq!(count, 18);
    }
}