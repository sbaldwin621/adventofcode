use thiserror::Error;

pub struct Config {
    pub filename: String,
    pub window_size: usize
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, ParseConfigError> {
        if args.len() < 3 {
            return Err(ParseConfigError::NotEnoughArguments);
        }

        let filename = args[1].clone();
        let window_size = args[2].parse::<usize>().map_err(|_| ParseConfigError::InvalidWindowSize)?;
        
        Ok(Config { filename, window_size })
    }
}

#[derive(Debug, Error)]
pub enum ParseConfigError {
    #[error("not enough arguments")]
    NotEnoughArguments,
    #[error("invalid window size")]
    InvalidWindowSize
}