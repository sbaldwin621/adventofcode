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

    pub fn solve(&mut self) -> Vec<BingoSolution> {
        let mut solutions = vec![];

        for board in self.boards.iter() {
            if let Some(solution) = board.solve(&self.chosen_numbers) {
                solutions.push(solution);
            }
        }

        solutions
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
    squares_map: HashMap<u64, usize>
}

impl BingoBoard {
    pub fn new(squares: Vec<u64>) -> BingoBoard {
        let mut squares_map = HashMap::new();
        for (i, &square) in squares.iter().enumerate() {
            squares_map.insert(square, i);
        }

        BingoBoard { squares, squares_map }
    }

    pub fn solve(&self, numbers: &Vec<u64>) -> Option<BingoSolution> {
        let mut marked = 0;

        for (turn, &number) in numbers.iter().enumerate() {
            if let Some(i) = self.squares_map.get(&number) {
                let mask = 1 << i;
                marked = marked | mask;

                if BingoBoard::has_bingo(marked) {
                    return Some(BingoSolution::new(turn + 1, self.score(marked) * number));
                }
            }
        }

        None
    }

    fn has_bingo(marked: u64) -> bool {
        for pattern in WINNING_PATTERNS {
            if marked & pattern == pattern {
                return true;
            }
        }

        false
    }

    fn score(&self, marked: u64) -> u64 {
        let mut score = 0;

        for (i, square) in self.squares.iter().enumerate() {
            if marked & (1 << i) == 0 {
                score += square;
            }
        }

        score
    }
}

#[derive(Debug)]
pub struct BingoSolution {
    pub turns_required: usize,
    pub score: u64
}

impl BingoSolution {
    pub fn new(turns_required: usize, score: u64) -> BingoSolution {
        BingoSolution { turns_required, score }
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

        // board.try_mark(8);
        // board.try_mark(2);
        // board.try_mark(23);
        // board.try_mark(4);
        // board.try_mark(24);

        // assert!(board.has_bingo());
    }
}