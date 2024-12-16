use std::str::FromStr;

use regex::Regex;

pub struct CraneGameList {
    crane_games: Vec<CraneGame>
}

impl CraneGameList {
    pub fn new(crane_games: Vec<CraneGame>) -> CraneGameList {
        CraneGameList { crane_games }
    }

    pub fn solve(&self) -> u32 {
        let mut total_tokens: u32 = 0;
        for (n, crane_game) in self.crane_games.iter().enumerate() {
            println!("solving {}", n);
            if let Some((a, b)) = crane_game.solve() {
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

            let a_x: u32 = a_x.parse().unwrap();
            let a_y: u32 = a_y.parse().unwrap();
            let b_x: u32 = b_x.parse().unwrap();
            let b_y: u32 = b_y.parse().unwrap();
            let prize_x: u32 = prize_x.parse().unwrap();
            let prize_y: u32 = prize_y.parse().unwrap();

            let crane_game = CraneGame::new((a_x, a_y), (b_x, b_y), (prize_x, prize_y));
            crane_games.push(crane_game);
        }

        Ok(CraneGameList::new(crane_games))
    }
}

pub struct CraneGame {
    button_a: (u32, u32),
    button_b: (u32, u32),
    prize: (u32, u32)
}

impl CraneGame {
    pub fn new(button_a: (u32, u32), button_b: (u32, u32), prize: (u32, u32)) -> CraneGame {
        CraneGame { button_a, button_b, prize }
    }

    pub fn solve(&self) -> Option<(u32, u32)> {
        let mut best_solution_cost = u32::MAX;
        let mut best_solution = None;

        for a_presses in (0..=100).rev() {
            for b_presses in    0..=100 {
                let cost = a_presses * 3 + b_presses;
                let (x, y) = self.simulate_move(a_presses, b_presses);
                if (x, y) == self.prize && cost < best_solution_cost {
                    best_solution_cost = cost;
                    best_solution = Some((a_presses, b_presses));
                }
            }
        }

        best_solution
    }
    
    fn simulate_move(&self, a_presses: u32, b_presses: u32) -> (u32, u32) {
        let (a_x, a_y) = self.button_a;
        let (b_x, b_y) = self.button_b;

        (a_x * a_presses + b_x * b_presses, a_y * a_presses + b_y * b_presses)
    }
}