use crate::problem::Problem;
use crate::util::read_file;

struct Command<'a> {
    instruction: &'a str,
    amount: i64,
}

pub struct Problem02 {}

impl Problem02 {
    fn solve_actual(&self, commands: &Vec<Command>) -> i64 {
        let mut depth: i64 = 0;
        let mut distance: i64 = 0;
        commands.iter().for_each(|cmd| match cmd.instruction {
            "forward" => distance += cmd.amount,
            "down" => depth += cmd.amount,
            "up" => depth -= cmd.amount,
            _ => (),
        });
        return depth * distance;
    }

    fn solve_actual_part2(&self, commands: &Vec<Command>) -> i64 {
        let mut depth: i64 = 0;
        let mut distance: i64 = 0;
        let mut aim: i64 = 0;
        commands.iter().for_each(|cmd| match cmd.instruction {
            "forward" => {
                distance += cmd.amount;
                depth += aim * cmd.amount;
            }
            "down" => aim += cmd.amount,
            "up" => aim -= cmd.amount,
            _ => (),
        });
        return depth * distance;
    }
}

impl Problem for Problem02 {
    fn new() -> Problem02 {
        Problem02 {}
    }

    fn solve(&self) {
        let input = match read_file("./inputs/problem_02.txt") {
            Some(data) => data,
            None => {
                println!("Day 2 Answer: Could not read input :(");
                return;
            }
        };
        let mut failed_to_parse = false;
        let commands: Vec<Command> = input
            .lines()
            .map(|line| {
                let mut split = line.split_ascii_whitespace();
                Command {
                    instruction: match split.next() {
                        Some(value) => value,
                        None => {
                            failed_to_parse = true;
                            ""
                        }
                    },
                    amount: match split.next() {
                        Some(value) => match value.parse::<i64>() {
                            Ok(parsed) => parsed,
                            Err(_e) => {
                                failed_to_parse = true;
                                -1
                            }
                        },
                        None => {
                            failed_to_parse = true;
                            -1
                        }
                    },
                }
            })
            .collect();

        if failed_to_parse {
            println!("Day 2 Answer: Could not parse input :(");
            return;
        }

        let result = self.solve_actual(&commands);
        let result_part2 = self.solve_actual_part2(&commands);
        println!("Day 2 Answer:");
        println!(" - Part 1: {}", result);
        println!(" - Part 2: {}", result_part2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem02::new();
        let commands: Vec<Command> = vec![
            Command {
                instruction: "forward",
                amount: 5,
            },
            Command {
                instruction: "down",
                amount: 5,
            },
            Command {
                instruction: "forward",
                amount: 8,
            },
            Command {
                instruction: "up",
                amount: 3,
            },
            Command {
                instruction: "down",
                amount: 8,
            },
            Command {
                instruction: "forward",
                amount: 2,
            },
        ];
        assert_eq!(problem.solve_actual(&commands), 150);
    }

    #[test]
    fn test_solve_actual_only_depth() {
        let problem = Problem02::new();
        let commands: Vec<Command> = vec![
            Command {
                instruction: "down",
                amount: 5,
            },
            Command {
                instruction: "down",
                amount: 5,
            },
            Command {
                instruction: "down",
                amount: 5,
            },
            Command {
                instruction: "up",
                amount: 5,
            },
        ];
        assert_eq!(problem.solve_actual(&commands), 0);
    }

    #[test]
    fn test_solve_actual_only_distance() {
        let problem = Problem02::new();
        let commands: Vec<Command> = vec![
            Command {
                instruction: "forward",
                amount: 5,
            },
            Command {
                instruction: "forward",
                amount: 5,
            },
            Command {
                instruction: "forward",
                amount: 5,
            },
            Command {
                instruction: "forward",
                amount: 5,
            },
        ];
        assert_eq!(problem.solve_actual(&commands), 0);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem02::new();
        let commands: Vec<Command> = vec![
            Command {
                instruction: "forward",
                amount: 5,
            },
            Command {
                instruction: "down",
                amount: 5,
            },
            Command {
                instruction: "forward",
                amount: 8,
            },
            Command {
                instruction: "up",
                amount: 3,
            },
            Command {
                instruction: "down",
                amount: 8,
            },
            Command {
                instruction: "forward",
                amount: 2,
            },
        ];
        assert_eq!(problem.solve_actual_part2(&commands), 900);
    }

    #[test]
    fn test_solve_actual_part2_only_depth() {
        let problem = Problem02::new();
        let commands: Vec<Command> = vec![
            Command {
                instruction: "down",
                amount: 5,
            },
            Command {
                instruction: "down",
                amount: 5,
            },
            Command {
                instruction: "down",
                amount: 5,
            },
            Command {
                instruction: "up",
                amount: 5,
            },
        ];
        assert_eq!(problem.solve_actual_part2(&commands), 0);
    }

    #[test]
    fn test_solve_actual_part2_only_distance() {
        let problem = Problem02::new();
        let commands: Vec<Command> = vec![
            Command {
                instruction: "forward",
                amount: 5,
            },
            Command {
                instruction: "forward",
                amount: 5,
            },
            Command {
                instruction: "forward",
                amount: 5,
            },
            Command {
                instruction: "forward",
                amount: 5,
            },
        ];
        assert_eq!(problem.solve_actual_part2(&commands), 0);
    }

    #[test]
    fn test_solve_actual_part2_big_movements() {
        let problem = Problem02::new();
        // Picked 2097151 because it's roughly the cube root of max(i64), so
        // this should be a worst case assuming input doesn't cause overflow.
        // Cube root because "down" will set aim to 2097151, then "forward"
        // will set distance to 2097151, and depth to aim * 2097151, then
        // both are multiplied together for the result.
        let commands: Vec<Command> = vec![
            Command {
                instruction: "down",
                amount: 2097151,
            },
            Command {
                instruction: "forward",
                amount: 2097151,
            },
        ];
        assert_eq!(problem.solve_actual_part2(&commands), 9223358842721533951);
    }
}
