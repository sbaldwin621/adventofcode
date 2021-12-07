use thiserror::Error;

pub struct Config {
    pub filename: String,
    pub generations: u64
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, ParseConfigError> {
        if args.len() < 3 {
            return Err(ParseConfigError::NotEnoughArguments);
        }

        let filename = args[1].clone();
        let generations = args[2].parse::<u64>()?;
        
        Ok(Config { filename, generations })
    }
}

#[derive(Debug, Error)]
pub enum ParseConfigError {
    #[error("not enough arguments")]
    NotEnoughArguments,
    
    #[error("couldn't parse generations")]
    ParseGenerationsError(#[from]std::num::ParseIntError)
}