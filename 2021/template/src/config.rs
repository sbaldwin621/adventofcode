use thiserror::Error;

pub struct Config {
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, ParseConfigError> {
        if args.len() < 2 {
            return Err(ParseConfigError::NotEnoughArguments);
        }

        let filename = args[1].clone();
        
        Ok(Config { filename })
    }
}

#[derive(Debug, Error)]
pub enum ParseConfigError {
    #[error("not enough arguments")]
    NotEnoughArguments
}