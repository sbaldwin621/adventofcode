use std::error::Error;
use std::fmt::Display;

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

#[derive(Debug)]
pub enum ParseConfigError {
    NotEnoughArguments
}

impl Display for ParseConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ParseConfigError::NotEnoughArguments => "not enough arguments"
        })
    }
}

impl Error for ParseConfigError { }