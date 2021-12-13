use std::collections::{HashMap, HashSet};

use crate::problem::Problem;

/// Converts an input string to a nice integral value.
///
/// Since we know all nodes are strings of length 2, or start or end,
/// u16 can easily hold enough info to differentiate nodes, and group
/// them by case.
fn value_as_num(value: &str) -> u16 {
    match value {
        "start" => u16::MIN,
        "end" => u16::MAX,
        _ => {
            let computed = value.chars().fold(0, |acc, c| {
                (acc << 8)
                    + (c as u16 + 1
                        - match c.is_ascii_lowercase() {
                            true => 'a' as u16,
                            false => 'A' as u16 - 26,
                        })
            });
            if computed <= 52 {
                computed << 8
            } else {
                computed
            }
        }
    }
}

pub struct Problem12 {}

impl Problem12 {
    pub fn new() -> Problem12 {
        Problem12 {}
    }

    fn parse(&self, input: String) -> HashMap<u16, Vec<u16>> {
        let mut edges: HashMap<u16, Vec<u16>> = HashMap::new();
        input.lines().for_each(|line| {
            let nodes: Vec<&str> = line.split('-').collect();
            let first = value_as_num(nodes[0]);
            let second = value_as_num(nodes[1]);
            if second != u16::MIN && first != u16::MAX {
                if edges.contains_key(&first) {
                    (*edges.get_mut(&first).unwrap()).push(second.clone());
                } else {
                    edges.insert(first.clone(), vec![second.clone()]);
                }
            }
            if first != u16::MIN && second != u16::MAX {
                if edges.contains_key(&second) {
                    (*edges.get_mut(&second).unwrap()).push(first);
                } else {
                    edges.insert(second, vec![first]);
                }
            }
        });
        edges
    }

    fn traverse_graph(
        &self,
        cave_paths: &HashMap<u16, Vec<u16>>,
        current_node: u16,
        seen_lowercase_nodes: HashSet<&u16>,
        had_duplicate_yet: bool,
    ) -> i64 {
        let mut seen_lowercase = seen_lowercase_nodes.clone();
        let mut paths = 0;
        /* value of zz from value_as_num */
        if current_node > u16::MIN && current_node <= 6682 {
            seen_lowercase.insert(&current_node);
        }
        for node in &cave_paths[&current_node] {
            let seen = seen_lowercase.contains(node);
            if node == &u16::MAX {
                paths += 1;
            } else if !seen || !had_duplicate_yet {
                paths += self.traverse_graph(
                    cave_paths,
                    *node,
                    seen_lowercase.clone(),
                    seen || had_duplicate_yet,
                );
            }
        }
        paths
    }

    fn solve_actual(&self, cave_paths: &HashMap<u16, Vec<u16>>) -> i64 {
        self.traverse_graph(cave_paths, u16::MIN, HashSet::new(), true)
    }

    fn solve_actual_part2(&self, cave_paths: &HashMap<u16, Vec<u16>>) -> i64 {
        self.traverse_graph(cave_paths, u16::MIN, HashSet::new(), false)
    }
}

impl Problem for Problem12 {
    fn name(&self) -> &str {
        "Day 12: Passage Pathing"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_12.txt");
        let cave_paths = self.parse(input);
        self.solve_actual(&cave_paths)
    }

    fn solve_part2(&self) -> i64 {
        let input = get_input!("./inputs/problem_12.txt");
        let cave_paths = self.parse(input);
        self.solve_actual_part2(&cave_paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example_01() {
        let problem = Problem12::new();
        let input = get_input!("./inputs/problem_12_example_01.txt");
        let cave_paths = problem.parse(input);
        assert_eq!(problem.solve_actual(&cave_paths), 10);
    }

    #[test]
    fn test_solve_actual_from_example_02() {
        let problem = Problem12::new();
        let input = get_input!("./inputs/problem_12_example_02.txt");
        let cave_paths = problem.parse(input);
        assert_eq!(problem.solve_actual(&cave_paths), 19);
    }

    #[test]
    fn test_solve_actual_from_example_03() {
        let problem = Problem12::new();
        let input = get_input!("./inputs/problem_12_example_03.txt");
        let cave_paths = problem.parse(input);
        assert_eq!(problem.solve_actual(&cave_paths), 226);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem12::new();
        let input = get_input!("./inputs/problem_12.txt");
        let cave_paths = problem.parse(input);
        assert_eq!(problem.solve_actual(&cave_paths), 4186);
    }

    #[test]
    fn test_solve_actual_part2_from_example_01() {
        let problem = Problem12::new();
        let input = get_input!("./inputs/problem_12_example_01.txt");
        let cave_paths = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&cave_paths), 36);
    }

    #[test]
    fn test_solve_actual_part2_from_example_02() {
        let problem = Problem12::new();
        let input = get_input!("./inputs/problem_12_example_02.txt");
        let cave_paths = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&cave_paths), 103);
    }

    #[test]
    fn test_solve_actual_part2_from_example_03() {
        let problem = Problem12::new();
        let input = get_input!("./inputs/problem_12_example_03.txt");
        let cave_paths = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&cave_paths), 3509);
    }

    #[test]
    fn test_solve_actual_part2_from_input() {
        let problem = Problem12::new();
        let input = get_input!("./inputs/problem_12.txt");
        let cave_paths = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&cave_paths), 92111);
    }
}
