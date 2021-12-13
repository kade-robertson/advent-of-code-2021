use std::collections::{HashMap, HashSet};

use crate::problem::Problem;

#[derive(Clone, Eq)]
pub struct CaveNode {
    value: u16,
    small: bool,
    start: bool,
    end: bool,
}

impl CaveNode {
    pub fn new(value: &str) -> CaveNode {
        CaveNode {
            value: value_as_num(value),
            small: value.chars().all(|c| c.is_ascii_lowercase()),
            start: value == "start".to_string(),
            end: value == "end".to_string(),
        }
    }
}

impl PartialEq for CaveNode {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl core::hash::Hash for CaveNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

fn value_as_num(value: &str) -> u16 {
    match value {
        "start" => u16::MIN,
        "end" => u16::MAX,
        _ => value.chars().fold(0, |acc, c| {
            (acc << 8)
                + (c as u16 + 1
                    - match c.is_ascii_lowercase() {
                        true => 'a' as u16,
                        false => 'A' as u16,
                    })
        }),
    }
}

pub struct Problem12 {}

impl Problem12 {
    pub fn new() -> Problem12 {
        Problem12 {}
    }

    fn parse(&self, input: String) -> HashMap<CaveNode, Vec<CaveNode>> {
        let mut edges: HashMap<CaveNode, Vec<CaveNode>> = HashMap::new();
        input.lines().for_each(|line| {
            let nodes: Vec<&str> = line.split('-').collect();
            let first = CaveNode::new(nodes[0]);
            let second = CaveNode::new(nodes[1]);
            if !second.start && !first.end {
                if edges.contains_key(&first) {
                    (*edges.get_mut(&first).unwrap()).push(second.clone());
                } else {
                    edges.insert(first.clone(), vec![second.clone()]);
                }
            }
            if !first.start && !second.end {
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
        cave_paths: &HashMap<CaveNode, Vec<CaveNode>>,
        current_node: &CaveNode,
        seen_lowercase_nodes: HashSet<&u16>,
        had_duplicate_yet: bool,
    ) -> i64 {
        let mut seen_lowercase = seen_lowercase_nodes.clone();
        let mut paths = 0;
        if current_node.small {
            seen_lowercase.insert(&current_node.value);
        }
        for node in &cave_paths[current_node] {
            let seen = seen_lowercase.contains(&node.value);
            if node.end {
                paths += 1;
            } else if !seen || !had_duplicate_yet {
                paths += self.traverse_graph(
                    cave_paths,
                    node,
                    seen_lowercase.clone(),
                    seen || had_duplicate_yet,
                );
            }
        }
        paths
    }

    fn solve_actual(&self, cave_paths: &HashMap<CaveNode, Vec<CaveNode>>) -> i64 {
        let start = cave_paths
            .iter()
            .find(|(node, _edges)| node.start)
            .unwrap()
            .0;
        self.traverse_graph(cave_paths, start, HashSet::new(), true)
    }

    fn solve_actual_part2(&self, cave_paths: &HashMap<CaveNode, Vec<CaveNode>>) -> i64 {
        let start = cave_paths
            .iter()
            .find(|(node, _edges)| node.start)
            .unwrap()
            .0;
        self.traverse_graph(cave_paths, start, HashSet::new(), false)
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
