use std::{fs::File, io::Read};

use crate::problem::Problem;

pub struct Problem01 {}

impl Problem01 {
    pub fn new() -> Problem01 {
        Problem01 {}
    }

    fn solve_actual(&self, measurements: &Vec<i64>) -> i64 {
        let mut increases = 0;
        let mut window = measurements.windows(2);
        while let Some([prev, next]) = window.next() {
            if next > prev {
                increases += 1;
            }
        }
        return increases;
    }

    fn solve_actual_part2(&self, measurements: &Vec<i64>) -> i64 {
        let mut increases = 0;
        let mut window = measurements.windows(4);
        while let Some([first, second, third, fourth]) = window.next() {
            if (second + third + fourth) > (first + second + third) {
                increases += 1;
            }
        }
        return increases;
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
        let result_part2 = self.solve_actual_part2(&measurements);
        println!("Day 1 Answer:");
        println!(" - Part 1: {}", result);
        println!(" - Part 2: {}", result_part2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem01::new();
        let measurements: Vec<i64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(problem.solve_actual(&measurements), 7);
    }

    #[test]
    fn test_solve_actual_always_increasing() {
        let problem = Problem01::new();
        let measurements: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(problem.solve_actual(&measurements), 6);
    }

    #[test]
    fn test_solve_actual_always_decreasing() {
        let problem = Problem01::new();
        let measurements: Vec<i64> = vec![7, 6, 5, 4, 3, 2, 1];
        assert_eq!(problem.solve_actual(&measurements), 0);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem01::new();
        let measurements: Vec<i64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(problem.solve_actual_part2(&measurements), 5);
    }

    #[test]
    fn test_solve_actual_part2_always_increasing() {
        let problem = Problem01::new();
        let measurements: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(problem.solve_actual_part2(&measurements), 4);
    }

    #[test]
    fn test_solve_actual_part2_always_decreasing() {
        let problem = Problem01::new();
        let measurements: Vec<i64> = vec![7, 6, 5, 4, 3, 2, 1];
        assert_eq!(problem.solve_actual_part2(&measurements), 0);
    }
}
