use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use clap::Parser;
use print_job::{OrderRule, OrderRuleSet, PageList, PrintJob};
use thiserror::Error;

mod print_job;

#[derive(Parser)]
pub struct CliOptions {
    part: u32,
    filename: std::path::PathBuf
}

enum PrintJobReadState {
    ExpectingOrderRule,
    ExpectingPageNumbers
}

pub fn run(options: CliOptions) -> Result<usize, ApplicationError> {
    let filename = options.filename;
   
    let mut state = PrintJobReadState::ExpectingOrderRule;
    let mut order_rules = vec![];
    let mut page_number_lists = vec![];

    let lines = read_lines(filename)?;
    for line in lines {
        let line = line?;

        state = match state {
            PrintJobReadState::ExpectingOrderRule => {
                if line.is_empty() {
                    PrintJobReadState::ExpectingPageNumbers
                } else {
                    let order_rule: OrderRule = line.parse().unwrap();
                    order_rules.push(order_rule);
                    
                    state
                }
            },
            PrintJobReadState::ExpectingPageNumbers => {
                let page_number_list: PageList = line.parse().unwrap();
                page_number_lists.push(page_number_list);

                state
            },
        }
    }

    let rule_set = OrderRuleSet::new(order_rules);
    let print_job = PrintJob::new(rule_set, page_number_lists);
    
    match options.part {
        1 => run_part1(&print_job),
        2 => run_part2(),
        _ => Err(ApplicationError::UnknownPart)
    }
}

fn run_part1(print_job: &PrintJob) -> Result<usize, ApplicationError> {
    Ok(print_job.calculate_score())
}

fn run_part2() -> Result<usize, ApplicationError> {
    !unimplemented!()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("unknown part")]
    UnknownPart,
    #[error("couldn't read puzzle input: {0}")]
    CouldntReadInput(#[from] io::Error)
}