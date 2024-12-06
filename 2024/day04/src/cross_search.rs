pub struct CrossSearch {
    input: String,
    row_size: usize
}

impl<'a> CrossSearch {
    pub fn new(input: String, row_size: usize) -> CrossSearch {
        CrossSearch { input, row_size }
    }

    pub fn search(&self) -> usize {
        let mut count = 0;

        let input = self.input.as_bytes();

        const M: u8 = 'M' as u8;
        const A: u8 = 'A' as u8;
        const S: u8 = 'S' as u8;

        let starting_index = self.row_size * 2 + 2;
        for i in starting_index..input.len() {
            if i % self.row_size > 1 {
                let bottom_right = input[i];
                let bottom_left = input[i - 2];
                let center = input[i - 1 - self.row_size];
                let top_left = input[i - 2 - self.row_size * 2];
                let top_right = input[i - self.row_size * 2];
                
                let is_match = match (top_left, top_right, center, bottom_left, bottom_right) {
                      (M, M, A, S, S)
                    | (M, S, A, M, S)
                    | (S, S, A, M, M)
                    | (S, M, A, S, M) => true,
                    _ => false
                };

                if is_match {
                    count += 1;
                } 
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input = String::from("MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX");
        let cross_search = CrossSearch::new(input, 10);
        let count = cross_search.search();

        assert_eq!(count, 9);
    }
}