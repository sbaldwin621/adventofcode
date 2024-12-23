use std::ops::Mul;

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
}
// 15940393
#[inline]
fn mix_prune(secret: u64, n: u64) -> u64 {
    (n ^ secret) % 16777216
}

#[cfg(test)]
mod tests {
    use super::SecretGenerator;

    #[test]
    pub fn example1() {
        let secrets = vec![123];
        let mut generator = SecretGenerator::new(secrets);

        generator.step();

        println!("{:?}", generator);
    }
}