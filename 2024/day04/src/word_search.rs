pub struct WordSearch {
    input: String,
    row_size: usize
}

impl<'a> WordSearch {
    pub fn new(input: String, row_size: usize) -> WordSearch {
        let padding = std::iter::repeat(".").take(row_size * 4).collect::<String>();
        let input = padding.clone() + &input + &padding;
        
        WordSearch { input, row_size }
    }

    pub fn search(&self) -> usize {
        let mut row_match_count = 0;
        let mut col_match_count = 0;
        let mut diag_match_count = 0;

        let input = self.input.as_bytes();

        const X: u8 = 'X' as u8;
        const M: u8 = 'M' as u8;
        const A: u8 = 'A' as u8;
        const S: u8 = 'S' as u8;

        let starting_index = self.row_size * 4;
        for i in starting_index..input.len() {
            let zero_index = i - 3 - self.row_size * 3;
            let zero_zero = input[zero_index];
            let zero_one = input[zero_index + 1];
            let zero_two = input[zero_index + 2];
            let zero_three = input[zero_index + 3];

            let one_index = i - 3 - self.row_size * 2;
            let one_zero = input[one_index];
            let one_one = input[one_index + 1];
            let one_two = input[one_index + 2];
            let one_three = input[one_index + 3];

            let two_index = i - 3 - self.row_size;
            let two_zero = input[two_index];
            let two_one = input[two_index + 1];
            let two_two = input[two_index + 2];
            let two_three = input[two_index + 3];

            let three_zero = input[i - 3];
            let three_one = input[i - 2];
            let three_two = input[i - 1];
            let three_three = input[i];                

            let grid = (
                (zero_zero, zero_one, zero_two, zero_three),
                (one_zero, one_one, one_two, one_three),
                (two_zero, two_one, two_two, two_three),
                (three_zero, three_one, three_two, three_three)
            );
            
            if i % self.row_size > 2 {
                let is_match = match (three_zero, three_one, three_two, three_three) {
                    (X, M, A, S)
                  | (S, A, M, X) => true,
                  _ => false
                };
    
                if is_match {
                    row_match_count += 1;
                }   
            }

            let is_match = match (zero_three, one_three, two_three, three_three) {
                  (X, M, A, S)
                | (S, A, M, X) => true,
                _ => false
            };

            if is_match {
                col_match_count += 1;
            }

            if i % self.row_size > 2 {
                let is_match = match grid {
                      ((X, _, _, _),
                       (_, M, _, _),
                       (_, _, A, _),
                       (_, _, _, S))
                    | ((S, _, _, _),
                       (_, A, _, _),
                       (_, _, M, _),
                       (_, _, _, X)) => true,
                    _ => false
                };

                if is_match {
                    diag_match_count += 1;
                }

                let is_match = match grid {
                    ((_, _, _, S),
                     (_, _, A, _),
                     (_, M, _, _),
                     (X, _, _, _))
                  | ((_, _, _, X),
                     (_, _, M, _),
                     (_, A, _, _),
                     (S, _, _, _)) => true,
                  _ => false
              };
              
              if is_match {
                  diag_match_count += 1;
              } 
            }
        }
        
        row_match_count + col_match_count + diag_match_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input = String::from("MMMSXXMASMMSAMXMSMSAAMXSXMAAMMMSAMASMSMXXMASAMXAMMXXAMMXXAMASMSMSASXSSSAXAMASAAAMAMMMXMMMMMXMXAXMASX");
        let word_search = WordSearch::new(input, 10);
        let count = word_search.search();

        assert_eq!(count, 18);
    }
}