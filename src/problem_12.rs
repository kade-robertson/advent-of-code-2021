use std::collections::{HashMap, HashSet};

use crate::problem::Problem;

#[derive(Clone, Eq)]
pub struct CaveNode {
    value: String,
    small: bool,
    start: bool,
    end: bool,
}

impl CaveNode {
    pub fn new(value: String) -> CaveNode {
        CaveNode {
            value: value.clone(),
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

pub struct Problem12 {}

impl Problem12 {
    pub fn new() -> Problem12 {
        Problem12 {}
    }

    fn parse(&self, input: String) -> HashMap<CaveNode, HashSet<CaveNode>> {
        let mut edges: HashMap<CaveNode, HashSet<CaveNode>> = HashMap::new();
        input.lines().for_each(|line| {
            let nodes: Vec<&str> = line.split('-').collect();
            let start = CaveNode::new(nodes[0].to_string());
            let end = CaveNode::new(nodes[1].to_string());
            if edges.contains_key(&start) {
                (*edges.get_mut(&start).unwrap()).insert(end.clone());
            } else {
                edges.insert(start.clone(), HashSet::from_iter([end.clone()]));
            }
            if edges.contains_key(&end) {
                (*edges.get_mut(&end).unwrap()).insert(start);
            } else {
                edges.insert(end, HashSet::from_iter([start]));
            }
        });
        edges
    }

    fn traverse_graph(
        &self,
        cave_paths: &HashMap<CaveNode, HashSet<CaveNode>>,
        current_node: &CaveNode,
        seen_lowercase_nodes: HashSet<&CaveNode>,
        had_duplicate_yet: bool,
    ) -> i64 {
        let mut seen_lowercase = seen_lowercase_nodes.clone();
        let mut paths = 0;
        if current_node.small {
            seen_lowercase.insert(current_node);
        }
        for node in &cave_paths[current_node] {
            if node.end {
                paths += 1;
            } else if node.start {
                continue;
            } else if !seen_lowercase.contains(node) || !had_duplicate_yet {
                paths += self.traverse_graph(
                    cave_paths,
                    node,
                    seen_lowercase.clone(),
                    seen_lowercase.contains(node) || had_duplicate_yet,
                );
            }
        }
        paths
    }

    fn solve_actual(&self, cave_paths: &HashMap<CaveNode, HashSet<CaveNode>>) -> i64 {
        let start = cave_paths
            .iter()
            .find(|(node, _edges)| node.start)
            .unwrap()
            .0;
        self.traverse_graph(cave_paths, start, HashSet::new(), true)
    }

    fn solve_actual_part2(&self, cave_paths: &HashMap<CaveNode, HashSet<CaveNode>>) -> i64 {
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
