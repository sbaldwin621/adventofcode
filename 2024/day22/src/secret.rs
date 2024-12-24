use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct MarketSimulator {
}

impl MarketSimulator {
    pub fn new() -> MarketSimulator {
        MarketSimulator { }
    }

    pub fn simulate(&self, secrets: Vec<u64>, generations: usize) -> MarketHistory {
        let mut secrets = secrets;
        
        let mut history = MarketHistory::new();
        for (i, secret) in secrets.iter().enumerate() {
            history.record_secret(i, secret);
        }

        for _ in 0..generations {
            for (i, secret) in secrets.iter_mut().enumerate() {
                *secret = mix_prune(*secret, *secret * 64);
                *secret = mix_prune(*secret, *secret >> 5);
                *secret = mix_prune(*secret, *secret * 2048);
                
                history.record_secret(i, secret);
            }
        }

        // for price in &history.price_histories.get(&0).unwrap().prices {
        //     println!("{} ({})", price.price, price.changes[3]);
        // }

        history
    }
}

#[derive(Debug)]
pub struct MarketHistory {
    price_histories: HashMap<usize, PriceHistory>,
    all_changes: HashSet<[i8;4]>
}

impl MarketHistory {
    pub fn new() -> MarketHistory {
        let price_histories = HashMap::new();
        let all_changes = HashSet::new();

        MarketHistory { price_histories, all_changes }
    }

    pub fn record_secret(&mut self, buyer_id: usize, secret: &u64) {
        let entry = self.price_histories.entry(buyer_id)
            .or_insert_with(|| PriceHistory::new());

        let price_point = entry.record_secret(*secret);
        self.all_changes.insert(price_point.changes.clone());
    }

    pub fn find_best_bananas(&self) -> i64 {
        let mut best_bananas = 0;

        for changes in self.all_changes.iter() {
            let bananas = self.count_bananas(changes);
            if bananas > best_bananas {
                best_bananas = bananas;
            }
        }

        best_bananas
    }

    pub fn count_bananas(&self, changes: &[i8;4]) -> i64 {
        let mut total_bananas: i64 = 0;

        for price_history in self.price_histories.values() {
            if let Some(price_point) = price_history.get_price_for_changes(changes) {
                total_bananas += price_point.price as i64;
            }
        }

        total_bananas
    }
}

#[derive(Debug)]
struct PriceHistory {
    prices: Vec<PricePoint>,
    first_change_occurences: HashMap<[i8;4], PricePoint>
}

impl PriceHistory {
    pub fn new() -> PriceHistory {
        let prices = vec![];
        let first_change_occurences = HashMap::new();

        PriceHistory { prices, first_change_occurences }
    }

    pub fn record_secret(&mut self, secret: u64) -> &PricePoint {
        let price: i8 = (secret % 10).try_into().unwrap();
        
        self.add_price(price)
    }

    pub fn add_price(&mut self, price: i8) -> &PricePoint {
        if self.prices.len() > 0 {
            let previous_price = &self.prices[self.prices.len() - 1];
            let new_price = previous_price.add(price);
            
            if self.prices.len() >= 4 {
                if !self.first_change_occurences.contains_key(&new_price.changes) {
                    self.first_change_occurences.insert(new_price.changes.clone(), new_price.clone());
                }
            }

            self.prices.push(new_price);
        } else {
            self.prices.push(PricePoint::new(price, [0, 0, 0, 0]));
        }

        &self.prices[self.prices.len() - 1]
    }

    pub fn get_price_for_changes(&self, changes: &[i8;4]) -> Option<&PricePoint> {
        self.first_change_occurences.get(changes)
    }
}

#[derive(Debug, Clone)]
struct PricePoint {
    price: i8,
    changes: [i8;4]
}

impl PricePoint {
    pub fn new(price: i8, changes: [i8;4]) -> PricePoint {
        PricePoint { price, changes }
    }

    pub fn add(&self, new_price: i8) -> PricePoint {
        let latest_change = new_price - self.price;
        let [_, b, c, d] = self.changes;
        let new_changes = [b, c, d, latest_change];

        PricePoint::new(new_price, new_changes)
    }
}

#[derive(Debug)]
pub struct SecretGenerator {
    secrets: Vec<u64>
}

impl SecretGenerator {
    pub fn new(secrets: Vec<u64>) -> SecretGenerator {
        SecretGenerator { secrets }
    }

    pub fn step(&mut self) {
        for secret in self.secrets.iter_mut() {
            *secret = mix_prune(*secret, *secret * 64);
            *secret = mix_prune(*secret, *secret >> 5);
            *secret = mix_prune(*secret, *secret * 2048);
        }
    }
    
    pub fn sum(&self) -> u64 {
        self.secrets.iter().sum()
    }

    pub fn secrets(&self) -> &Vec<u64> {
        &self.secrets
    }
}

#[inline]
fn mix_prune(secret: u64, n: u64) -> u64 {
    (n ^ secret) % 16777216
}

#[cfg(test)]
mod tests {
    use crate::secret::MarketSimulator;

    use super::SecretGenerator;

    #[test]
    pub fn part1_example1() {
        let secrets = vec![123];
        let mut generator = SecretGenerator::new(secrets);

        generator.step();

        println!("{:?}", generator);
    }

    #[test]
    pub fn part2_example1() {
        let secrets = vec![123];
        let market = MarketSimulator::new();

        market.simulate(secrets, 10);
    }

    #[test]
    pub fn part2_example2() {
        let secrets = vec![1, 2, 3, 2024];
        let market = MarketSimulator::new();

        let history = market.simulate(secrets, 2000);

        assert_eq!(history.find_best_bananas(), 23);
    }
}