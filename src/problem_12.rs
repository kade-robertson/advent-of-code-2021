use std::collections::{HashMap, HashSet};

use crate::problem::Problem;

pub struct Problem12 {}

impl Problem12 {
    pub fn new() -> Problem12 {
        Problem12 {}
    }

    fn parse(&self, input: String) -> HashMap<String, HashSet<String>> {
        let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
        input.lines().for_each(|line| {
            let nodes: Vec<&str> = line.split('-').collect();
            let start = nodes[0].to_string();
            let end = nodes[1].to_string();
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
        cave_paths: &HashMap<String, HashSet<String>>,
        current_node: &String,
        seen_lowercase_nodes: HashSet<&String>,
    ) -> i64 {
        let mut seen_lowercase = seen_lowercase_nodes.clone();
        let mut paths = 0;
        if current_node.chars().all(|c| c.is_ascii_lowercase()) {
            seen_lowercase.insert(current_node);
        }
        for node in &cave_paths[current_node] {
            if *node == "end".to_string() {
                paths += 1;
            } else if !seen_lowercase.contains(node) {
                paths += self.traverse_graph(cave_paths, node, seen_lowercase.clone());
            }
        }
        paths
    }

    fn solve_actual(&self, cave_paths: &HashMap<String, HashSet<String>>) -> i64 {
        let start = "start".to_string();
        self.traverse_graph(cave_paths, &start, HashSet::from_iter([&start]))
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
        0
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
}
