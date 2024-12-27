use std::str::FromStr;

use regex::Regex;

pub struct CraneGameList {
    crane_games: Vec<CraneGame>
}

impl CraneGameList {
    pub fn new(crane_games: Vec<CraneGame>) -> CraneGameList {
        CraneGameList { crane_games }
    }

    pub fn solve(&self, adjustment: i64) -> i64 {
        let mut total_tokens: i64 = 0;
        for (n, crane_game) in self.crane_games.iter().enumerate() {
            println!("solving {:?}", crane_game);
            if let Some((a, b)) = crane_game.solve(adjustment) {
                println!("found solution A: {}, B: {}", a, b);
                total_tokens += a * 3 + b;
            } else {
                println!("no solution");
            }
        }

        total_tokens
    }
}

impl FromStr for CraneGameList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();

        let mut crane_games = vec![];

        for captures in pattern.captures_iter(s) {
            let (_, [a_x, a_y, b_x, b_y, prize_x, prize_y]) = captures.extract();

            let a_x: i64 = a_x.parse().unwrap();
            let a_y: i64 = a_y.parse().unwrap();
            let b_x: i64 = b_x.parse().unwrap();
            let b_y: i64 = b_y.parse().unwrap();
            let prize_x: i64 = prize_x.parse().unwrap();
            let prize_y: i64 = prize_y.parse().unwrap();

            let crane_game = CraneGame::new((a_x, a_y), (b_x, b_y), (prize_x, prize_y));
            crane_games.push(crane_game);
        }

        Ok(CraneGameList::new(crane_games))
    }
}

#[derive(Debug)]
pub struct CraneGame {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64)
}

impl CraneGame {
    pub fn new(button_a: (i64, i64), button_b: (i64, i64), prize: (i64, i64)) -> CraneGame {
        CraneGame { button_a, button_b, prize }
    }

    pub fn solve(&self, adjustment: i64) -> Option<(i64, i64)> {
        let (a_x, a_y) = self.button_a;
        let (b_x, b_y) = self.button_b;
        
        let target_x = self.prize.0 + adjustment;
        let target_y = self.prize.1 + adjustment;
        
        let b = (target_y * a_x - target_x * a_y) / (b_y * a_x - a_y * b_x);
        let a = (target_x - b_x * b) / a_x;
        
        let result_x = a_x * a + b_x * b;
        let result_y = a_y * a + b_y * b;
        if result_x == target_x && result_y == target_y {
            Some((a, b))
        } else {
            None
        }
    }
}