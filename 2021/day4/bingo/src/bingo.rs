use std::{collections::{HashMap}, io::{self, BufRead}, path::Path, fs::File};

use crate::ApplicationError;

const WINNING_PATTERNS: [u64; 10] = [
    0b00000_00000_00000_00000_11111,
    0b00000_00000_00000_11111_00000,
    0b00000_00000_11111_00000_00000,
    0b00000_11111_00000_00000_00000,
    0b11111_00000_00000_00000_00000,
    0b00001_00001_00001_00001_00001,
    0b00010_00010_00010_00010_00010,
    0b00100_00100_00100_00100_00100,
    0b01000_01000_01000_01000_01000,
    0b10000_10000_10000_10000_10000
];

#[derive(Debug)]
pub struct PuzzleInput {
    chosen_numbers: Vec<u64>,
    boards: Vec<BingoBoard>    
}

impl PuzzleInput {
    pub fn new(chosen_numbers: Vec<u64>, boards: Vec<BingoBoard>) -> PuzzleInput {
        PuzzleInput { chosen_numbers, boards }
    }

    pub fn load_from_file(path: &String) -> Result<PuzzleInput, ApplicationError> {
        let mut lines = read_lines(path)?.into_iter();
    
        let first_line = lines.next().unwrap().unwrap();
        let numbers: Vec<u64> = first_line.split(",").map(|n| n.parse::<u64>().unwrap()).collect();
        
        let mut boards = vec![];
        while lines.next().is_some() {
            let mut squares = vec![];
            for _ in 0..5 {
                let line = lines.next().unwrap().unwrap();
                let numbers: Vec<u64> = line.split(" ").filter(|s| s.len() > 0).map(|n| n.parse::<u64>().unwrap()).collect();
                squares.extend(numbers);
            }
    
            let board = BingoBoard::new(squares);
            boards.push(board);
        }
        
        Ok(PuzzleInput::new(numbers, boards))
    }

    pub fn solve(&mut self) -> u64 {
        for &number in self.chosen_numbers.iter() {
            for board in self.boards.iter_mut() {
                board.try_mark(number);
                if board.has_bingo() {
                    return board.score() * number;
                }
            }
        }
        
        0
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
pub struct BingoBoard {
    squares: Vec<u64>,
    squares_map: HashMap<u64, usize>,
    marked: u64
}

impl BingoBoard {
    pub fn new(squares: Vec<u64>) -> BingoBoard {
        let mut squares_map = HashMap::new();
        for (i, &square) in squares.iter().enumerate() {
            squares_map.insert(square, i);
        }

        BingoBoard { squares, squares_map, marked: 0 }
    }

    pub fn try_mark(&mut self, number: u64) -> bool {
        if let Some(i) = self.squares_map.get(&number) {
            let mask = 1 << i;
            self.marked = self.marked | mask;

            true
        } else {
            false
        }
    }

    pub fn has_bingo(&self) -> bool {
        for pattern in WINNING_PATTERNS {
            if self.marked & pattern == pattern {
                return true;
            }
        }

        false
    }

    pub fn score(&self) -> u64 {
        let mut score = 0;

        for (i, square) in self.squares.iter().enumerate() {
            if self.marked & (1 << i) == 0 {
                score += square;
            }
        }

        score   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let mut board = BingoBoard::new(vec![
            22, 13, 17, 11,  0,
             8,  2, 23,  4, 24,
            21,  9, 14, 16,  7,
             6, 10,  3, 18,  5,
             1, 12, 20, 15, 19
        ]);

        board.try_mark(8);
        board.try_mark(2);
        board.try_mark(23);
        board.try_mark(4);
        board.try_mark(24);

        assert!(board.has_bingo());
    }
}