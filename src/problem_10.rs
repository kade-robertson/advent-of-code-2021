use std::collections::HashMap;

use crate::problem::Problem;

pub struct Problem10 {}

impl Problem10 {
    pub fn new() -> Problem10 {
        Problem10 {}
    }

    fn parse(&self, input: String) -> Vec<String> {
        input
            .lines()
            .map(|line| line.split_whitespace().collect())
            .collect()
    }

    fn solve_actual(&self, navigation_subsystem: &Vec<String>) -> i64 {
        let closed_map: HashMap<char, char> =
            HashMap::from_iter([(')', '('), ('}', '{'), (']', '['), ('>', '<')]);
        let scoring: HashMap<char, i64> =
            HashMap::from_iter([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

        let mut score = 0;
        for line in navigation_subsystem {
            let mut char_stack: Vec<char> = Vec::new();
            let mut corrupt = false;
            for c in line.chars() {
                if corrupt {
                    break;
                }
                match c {
                    '(' | '[' | '{' | '<' => char_stack.push(c),
                    ')' | ']' | '}' | '>' => {
                        if char_stack.pop().unwrap() != closed_map[&c] {
                            score += scoring[&c];
                            corrupt = true;
                        }
                    }
                    _ => (),
                }
            }
        }
        return score;
    }
}

impl Problem for Problem10 {
    fn name(&self) -> &str {
        "Day 10: Syntax Scoring"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_10.txt");
        let navigation_subsystem = self.parse(input);
        return self.solve_actual(&navigation_subsystem);
    }

    fn solve_part2(&self) -> i64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem10::new();
        let input = get_input!("./inputs/problem_10_example.txt");
        let navigation_subsystem = problem.parse(input);
        assert_eq!(problem.solve_actual(&navigation_subsystem), 26397);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem10::new();
        let input = get_input!("./inputs/problem_10.txt");
        let navigation_subsystem = problem.parse(input);
        assert_eq!(problem.solve_actual(&navigation_subsystem), 318099);
    }
}
