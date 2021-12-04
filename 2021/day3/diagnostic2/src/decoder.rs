pub enum DecoderMode {
    Oxygen,
    CO2
}

pub struct Decoder { }

impl Decoder {
    pub fn new() -> Decoder {
        Decoder { }
    }

    pub fn find_life_support_rating(&self, values: &Vec<String>) -> u64 {
        let oxygen = self.find_oxygen_rating(values);
        let co2 = self.find_co2_rating(values);

        oxygen * co2
    }
    
    pub fn find_oxygen_rating(&self, values: &Vec<String>) -> u64 {
        self.find_rating(values, DecoderMode::Oxygen)
    }

    pub fn find_co2_rating(&self, values: &Vec<String>) -> u64 {
        self.find_rating(values, DecoderMode::CO2)
    }

    fn find_rating(&self, values: &Vec<String>, mode: DecoderMode) -> u64 {
        let mut i: usize = 0;

        let mut current_values = vec![];
        for value in values.iter() {
            current_values.push(value);
        }

        loop {
            if current_values.len() == 1 {
                return u64::from_str_radix( current_values.first().unwrap(), 2).unwrap();
            }

            let mut zeros = vec![];
            let mut ones = vec![];

            let mut counter = 0;
            for value in current_values.iter() {
                match value.chars().nth(i) {
                    Some('0') => {
                        counter -= 1;
                        zeros.push(value.clone());
                    },
                    Some('1') => {
                        counter += 1;
                        ones.push(value.clone());
                    }
                    _ => panic!(":(")
                };
            }
            
            current_values = if counter >= 0 {
                match mode {
                    DecoderMode::Oxygen => ones,
                    DecoderMode::CO2 => zeros
                }
            } else {
                match mode {
                    DecoderMode::Oxygen => zeros,
                    DecoderMode::CO2 => ones
                }
            };

            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut decoder = Decoder::new();

        let co2 = decoder.find_life_support_rating(&vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string()
        ]);

        println!("{}", co2);
    }
}