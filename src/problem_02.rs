use crate::problem::Problem;

struct Command {
    instruction: String,
    amount: i64,
}

pub struct Problem02 {}

impl Problem02 {
    pub fn new() -> Problem02 {
        Problem02 {}
    }

    fn parse(&self, input: String) -> Vec<Command> {
        let mut commands: Vec<Command> = Vec::new();
        input.lines().for_each(|line| {
            let split: Vec<&str> = line.split_ascii_whitespace().collect();
            commands.push(Command {
                instruction: split[0].to_string(),
                amount: split[1].parse::<i64>().unwrap(),
            });
        });
        commands
    }

    fn solve_actual(&self, commands: &Vec<Command>) -> i64 {
        let mut depth: i64 = 0;
        let mut distance: i64 = 0;
        commands
            .iter()
            .for_each(|cmd| match cmd.instruction.as_str() {
                "forward" => distance += cmd.amount,
                "down" => depth += cmd.amount,
                "up" => depth -= cmd.amount,
                _ => (),
            });
        depth * distance
    }

    fn solve_actual_part2(&self, commands: &Vec<Command>) -> i64 {
        let mut depth: i64 = 0;
        let mut distance: i64 = 0;
        let mut aim: i64 = 0;
        commands
            .iter()
            .for_each(|cmd| match cmd.instruction.as_str() {
                "forward" => {
                    distance += cmd.amount;
                    depth += aim * cmd.amount;
                }
                "down" => aim += cmd.amount,
                "up" => aim -= cmd.amount,
                _ => (),
            });
        depth * distance
    }
}

impl Problem for Problem02 {
    fn name(&self) -> &str {
        "Day 2: Dive!"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_02.txt");
        let commands: Vec<Command> = self.parse(input);
        self.solve_actual(&commands)
    }

    fn solve_part2(&self) -> i64 {
        let input = get_input!("./inputs/problem_02.txt");
        let commands: Vec<Command> = self.parse(input);
        self.solve_actual_part2(&commands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem02::new();
        let input = get_input!("./inputs/problem_02_example.txt");
        let commands = problem.parse(input);
        assert_eq!(problem.solve_actual(&commands), 150);
    }

    #[test]
    fn test_solve_actual_only_depth() {
        let problem = Problem02::new();
        let commands: Vec<Command> = vec![
            Command {
                instruction: "down".to_string(),
                amount: 5,
            },
            Command {
                instruction: "down".to_string(),
                amount: 5,
            },
            Command {
                instruction: "down".to_string(),
                amount: 5,
            },
            Command {
                instruction: "up".to_string(),
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
                instruction: "forward".to_string(),
                amount: 5,
            },
            Command {
                instruction: "forward".to_string(),
                amount: 5,
            },
            Command {
                instruction: "forward".to_string(),
                amount: 5,
            },
            Command {
                instruction: "forward".to_string(),
                amount: 5,
            },
        ];
        assert_eq!(problem.solve_actual(&commands), 0);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem02::new();
        let input = get_input!("./inputs/problem_02_example.txt");
        let commands = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&commands), 900);
    }

    #[test]
    fn test_solve_actual_part2_only_depth() {
        let problem = Problem02::new();
        let commands: Vec<Command> = vec![
            Command {
                instruction: "down".to_string(),
                amount: 5,
            },
            Command {
                instruction: "down".to_string(),
                amount: 5,
            },
            Command {
                instruction: "down".to_string(),
                amount: 5,
            },
            Command {
                instruction: "up".to_string(),
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
                instruction: "forward".to_string(),
                amount: 5,
            },
            Command {
                instruction: "forward".to_string(),
                amount: 5,
            },
            Command {
                instruction: "forward".to_string(),
                amount: 5,
            },
            Command {
                instruction: "forward".to_string(),
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
                instruction: "down".to_string(),
                amount: 2097151,
            },
            Command {
                instruction: "forward".to_string(),
                amount: 2097151,
            },
        ];
        assert_eq!(problem.solve_actual_part2(&commands), 9223358842721533951);
    }
}
