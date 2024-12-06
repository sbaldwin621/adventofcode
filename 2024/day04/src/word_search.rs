pub struct WordSearch {
    input: String,
    row_size: usize
}

impl<'a> WordSearch {
    pub fn new(input: String, row_size: usize) -> WordSearch {
        WordSearch { input, row_size }
    }

    pub fn search(&self, target: &String) -> usize {
        let mut count = 0;

        let input = self.input.as_bytes();
        let target = target.as_bytes();
        let target_reversed: Vec<u8> = target.iter().rev().cloned().collect();
        
        count += self.search_impl(input, target, 1, 0);
        count += self.search_impl(input, &target_reversed, 1, 0);
        
        count += self.search_impl(input, target, self.row_size, 1);
        count += self.search_impl(input, &target_reversed, self.row_size, 1);

        count += self.search_impl(input, target, self.row_size + 1, 1);
        count += self.search_impl(input, &target_reversed, self.row_size + 1, 1);

        count += self.search_impl(input, target, self.row_size - 1, 1);
        count += self.search_impl(input, &target_reversed, self.row_size - 1, 1);

        count
    }
    
    fn search_impl(&self, input: &[u8], target: &[u8], step: usize, expected_row_difference: usize) -> usize {
        let mut count = 0;

        for index in 0..input.len() {
            if input[index] == target[0] {
                let next = index + step;

                let current_row = index / self.row_size;
                let next_row = next / self.row_size;

                if next_row - current_row == expected_row_difference {
                    count += self.match_rest(input, &target[1..], next, step, expected_row_difference);
                }
            }
        }

        count
    }

    fn match_rest(&self, input: &[u8], target: &[u8], index: usize, step: usize, expected_row_difference: usize) -> usize {
        if index >= input.len() {
            return 0;
        }

        if input[index] == target[0] {
            if target.len() == 1 {
                return 1;
            }

            let next = index + step;

            let current_row = index / self.row_size;
            let next_row = next / self.row_size;

            if next_row - current_row == expected_row_difference {
                return self.match_rest(input, &target[1..], next, step, expected_row_difference);
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input = String::from("MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX");
        let word_search = WordSearch::new(input, 10);
        let count = word_search.search(&String::from("XMAS"));

        assert_eq!(count, 18);
    }
}