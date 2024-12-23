use std::collections::HashMap;

#[derive(Debug)]
pub struct MarketSimulator {
}

impl MarketSimulator {
    pub fn new() -> MarketSimulator {
        MarketSimulator { }
    }

    pub fn simulate(&self, secrets: Vec<u64>, generations: usize) {
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

        for price in &history.price_histories.get(&0).unwrap().prices {
            println!("{} ({})", price.price, price.changes[3]);
        }
    }
}

#[derive(Debug)]
struct MarketHistory {
    price_histories: HashMap<usize, PriceHistory>
}

impl MarketHistory {
    pub fn new() -> MarketHistory {
        let price_histories = HashMap::new();

        MarketHistory { price_histories }
    }

    pub fn record_secret(&mut self, buyer_id: usize, secret: &u64) {
        let entry = self.price_histories.entry(buyer_id)
            .or_insert_with(|| PriceHistory::new());

        entry.record_secret(*secret);
    }
}

#[derive(Debug)]
struct PriceHistory {
    prices: Vec<PricePoint>
}

impl PriceHistory {
    pub fn new() -> PriceHistory {
        let prices = vec![];

        PriceHistory { prices }
    }

    pub fn record_secret(&mut self, secret: u64) {
        let price: i8 = (secret % 10).try_into().unwrap();
        self.add_price(price);
    }

    pub fn add_price(&mut self, price: i8) {
        if self.prices.len() > 0 {
            let previous_price = &self.prices[self.prices.len() - 1];
            self.prices.push(previous_price.add(price));
        } else {
            self.prices.push(PricePoint::new(price, [0, 0, 0, 0]));
        }
    }
}

#[derive(Debug)]
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
}