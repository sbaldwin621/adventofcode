pub struct Decoder {
    tracker: Vec<i64>
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder { tracker: vec![] }
    }

    pub fn ingest(&mut self, value: u64) {
        let mut i = 0;
        let mut remaining_value = value;

        while remaining_value > 0 || i < self.tracker.len() {
            if i >= self.tracker.len() {
                self.tracker.push(0);
            }

            let current_digit = remaining_value & 1;
            if current_digit == 1 {
                self.tracker[i] += 1;
            } else {
                self.tracker[i] -= 1;
            }

            remaining_value = remaining_value >> 1;

            i += 1;
        }
    }

    pub fn calculate_power_consumption(&self) -> u64 {
        let mut gamma = 0;
        let mask = 2u64.pow(self.tracker.len().try_into().unwrap()) - 1;

        for i in 0..self.tracker.len() {
            if self.tracker[i] > 0 {
                gamma += 1 << i;
            }
        }

        // epsilon is the negation of gamma (but the same number of bits)
        let epsilon = !gamma & mask;

        gamma * epsilon
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut decoder = Decoder::new();

        decoder.ingest(0b00100);
        decoder.ingest(0b11110);
        decoder.ingest(0b10110);
        decoder.ingest(0b10111);
        decoder.ingest(0b10101);
        decoder.ingest(0b01111);
        decoder.ingest(0b00111);
        decoder.ingest(0b11100);
        decoder.ingest(0b10000);
        decoder.ingest(0b11001);
        decoder.ingest(0b00010);
        decoder.ingest(0b01010);

        assert_eq!(198, decoder.calculate_power_consumption());
    }
}