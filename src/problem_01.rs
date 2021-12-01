use std::{fs::File, io::Read};

use crate::problem::Problem;

pub struct Problem01 {}

impl Problem01 {
    pub fn new() -> Problem01 {
        Problem01 {}
    }

    fn solve_actual(&self, _measurements: &Vec<i64>) -> i64 {
        return 0;
    }
}

impl Problem for Problem01 {
    fn solve(&self) {
        let mut file: File = match File::open("./inputs/problem_01.txt") {
            Ok(valid_file) => valid_file,
            Err(_e) => {
                println!("Day 1 Answer: Could not open input :(");
                return;
            }
        };

        let mut input = String::new();
        match file.read_to_string(&mut input) {
            Ok(_valid) => {}
            Err(_e) => {
                println!("Day 1 Answer: Could not read input :(");
                return;
            }
        }

        let mut failed_to_parse = false;
        let measurements: Vec<i64> = input
            .lines()
            .map(|line| match line.parse::<i64>() {
                Ok(value) => value,
                Err(_e) => {
                    failed_to_parse = true;
                    return -1;
                }
            })
            .collect();
        if failed_to_parse {
            println!("Day 1 Answer: Could not parse input :(");
            return;
        }

        let result = self.solve_actual(&measurements);
        println!("Day 1 Answer: {}", result);
    }
}
