use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use crate::problem::Problem;

fn neighbors(row: usize, col: usize, size: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if row > 0 {
        neighbors.push((row - 1, col));
    }
    if col > 0 {
        neighbors.push((row, col - 1));
    }
    if row < size - 1 {
        neighbors.push((row + 1, col));
    }
    if col < size - 1 {
        neighbors.push((row, col + 1));
    }

    neighbors
}

pub struct Problem15 {}

impl Problem15 {
    pub fn new() -> Problem15 {
        Problem15 {}
    }

    fn parse(&self, input: String) -> Vec<Vec<i64>> {
        let mut grid = Vec::new();
        input.lines().for_each(|line| {
            grid.push(
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i64)
                    .collect::<Vec<i64>>(),
            )
        });
        grid
    }

    fn expand_grid(&self, risk_levels: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
        let mut new_grid = vec![vec![-1; risk_levels.len() * 5]; risk_levels.len() * 5];

        for row in 0..risk_levels.len() {
            for col in 0..risk_levels.len() {
                for nrow in 0..5usize {
                    for ncol in 0..5usize {
                        let new_risk = risk_levels[row][col] + nrow as i64 + ncol as i64;
                        new_grid[(nrow * risk_levels.len()) + row]
                            [(ncol * risk_levels.len()) + col] = match new_risk > 9 {
                            true => new_risk % 10 + 1,
                            false => new_risk,
                        }
                    }
                }
            }
        }

        new_grid
    }

    fn get_lowest_risk_cost(&self, risk_levels: &Vec<Vec<i64>>) -> i64 {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut estimated_costs: Vec<Vec<i64>> =
            vec![vec![i64::MAX; risk_levels.len()]; risk_levels.len()];
        estimated_costs[0][0] = 0;

        // Need to use Reverse<> to act as a min-heap
        let mut to_visit: BinaryHeap<(Reverse<i64>, usize, usize)> = BinaryHeap::new();
        to_visit.push((Reverse(0), 0, 0));

        while let Some((_score, row, col)) = to_visit.pop() {
            visited.insert(row * risk_levels.len() + col);

            for (nrow, ncol) in neighbors(row, col, risk_levels.len()) {
                let poshash = nrow * risk_levels.len() + ncol;
                if visited.contains(&poshash) {
                    continue;
                }
                let new_cost = estimated_costs[row][col] + risk_levels[nrow][ncol];
                if new_cost < estimated_costs[nrow][ncol] {
                    estimated_costs[nrow][ncol] = new_cost;
                    to_visit.push((Reverse(new_cost), nrow, ncol));
                }
            }
        }

        estimated_costs[risk_levels.len() - 1][risk_levels.len() - 1]
    }

    fn solve_actual(&self, risk_levels: &Vec<Vec<i64>>) -> i64 {
        self.get_lowest_risk_cost(risk_levels)
    }

    fn solve_actual_part2(&self, risk_levels: &Vec<Vec<i64>>) -> i64 {
        self.get_lowest_risk_cost(&self.expand_grid(risk_levels))
    }
}

impl Problem for Problem15 {
    fn name(&self) -> &str {
        "Day 15: Chiton"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_15.txt");
        let risk_levels = self.parse(input);
        self.solve_actual(&risk_levels)
    }

    fn solve_part2(&self) -> (i64, Option<String>) {
        let input = get_input!("./inputs/problem_15.txt");
        let risk_levels = self.parse(input);
        (self.solve_actual_part2(&risk_levels), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem15::new();
        let input = get_input!("./inputs/problem_15_example.txt");
        let risk_levels = problem.parse(input);
        assert_eq!(problem.solve_actual(&risk_levels), 40);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem15::new();
        let input = get_input!("./inputs/problem_15.txt");
        let risk_levels = problem.parse(input);
        assert_eq!(problem.solve_actual(&risk_levels), 503);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem15::new();
        let input = get_input!("./inputs/problem_15_example.txt");
        let risk_levels = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&risk_levels), 315);
    }

    #[test]
    fn test_solve_actual_part2_from_input() {
        let problem = Problem15::new();
        let input = get_input!("./inputs/problem_15.txt");
        let risk_levels = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&risk_levels), 2853);
    }
}
