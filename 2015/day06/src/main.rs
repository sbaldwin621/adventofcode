use std::process;

use clap::Parser;

use advent_of_code_2015_day_6::{run, CliOptions};

fn main() {
    let cli_options = CliOptions::parse();
    
    let result = run(cli_options).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });
    
    println!("{}", result);
}
