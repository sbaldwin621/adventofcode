use std::collections::HashSet;
use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct CliOptions {
    filename: std::path::PathBuf
}

pub fn run(options: CliOptions) -> Result<u32, ApplicationError> {
    let filename = options.filename;

    let mut total = 0;

    let lines = read_lines(filename)?;
    for line in lines {
        let split: Vec<String> = line?.split("|").take(2).map(|s| s.to_string()).collect();
        let digits: Vec<HashSet<char>> = split[0].split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect();
        let output: Vec<HashSet<char>> = split[1].split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.chars().collect::<HashSet<char>>())
            .collect();

        let mut one = None;
        let mut four = None;
        let mut seven = None;
        let mut eight = None;

        let mut remaining_digits: Vec<HashSet<char>> = Vec::new();

        for digit in digits {
            match digit.len() {
                2 => { 
                    // 1
                    one = Some(digit);
                }
                4 => {
                    // 4
                    four = Some(digit);
                }
                3 => {
                    // 7
                    seven = Some(digit);
                }
                7 => {
                    // 8
                    eight = Some(digit);
                }
                _ => { 
                    remaining_digits.push(digit);
                }
            }
        }

        let one = one.expect("didn't find unique 1");
        let four = four.expect("didn't find unique 4");
        let seven = seven.expect("didn't find unique 7");
        let eight = eight.expect("didn't find unique 8");

        let mut two = None;
        let mut three = None;
        let mut five = None;
        let mut six = None;
        let mut nine = None;
        let mut zero = None;

        for digit in remaining_digits {
            match digit.len() {
                5 => {
                    // 2, 3, 5
                    if digit.is_superset(&one) {
                        three = Some(digit);
                    } else {
                        match digit.intersection(&four).count() {
                            2 => {
                                two = Some(digit);
                            }
                            3 => {
                                five = Some(digit);
                            }
                            _ => {
                                panic!("uh oh");
                            }
                        }
                    }
                }
                6 => {
                    // 0, 6, 9
                    if digit.is_superset(&four) {
                        nine = Some(digit);
                    } else {
                        match digit.intersection(&one).count() {
                            1 => {
                                six = Some(digit);
                            }
                            2 => {
                                zero = Some(digit);
                            }
                            _ => {
                                panic!("uh oh");
                            }
                        }
                    }
                }
                _ => {
                    panic!("unexpected digit");
                }
            }
        }

        let two = two.expect("didn't find unique 2");
        let three = three.expect("didn't find unique 3");
        let five = five.expect("didn't find unique 5");
        let six = six.expect("didn't find unique 6");
        let nine = nine.expect("didn't find unique 9");
        let zero = zero.expect("didn't find unique 0");

        let mut output_value = 0;

        for digit in output {
            output_value *= 10;

            let value = if digit == zero {
                0
            } else if digit == one {
                1
            } else if digit == two {
                2
            } else if digit == three {
                3
            } else if digit == four {
                4
            } else if digit == five {
                5
            } else if digit == six {
                6
            } else if digit == seven {
                7
            } else if digit == eight {
                8
            } else if digit == nine {
                9
            } else {
                panic!("uh oh")
            };

            output_value += value;
        }

        total += output_value;
    }

    Ok(total)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}