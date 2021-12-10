use std::collections::{HashSet, VecDeque};

use crate::problem::Problem;

pub struct Problem09 {}

impl Problem09 {
    pub fn new() -> Problem09 {
        Problem09 {}
    }

    fn parse(&self, input: String) -> Vec<Vec<i64>> {
        input
            .lines()
            .map(|line| line.chars().map(|c| (c as i64) - 48).collect())
            .collect()
    }

    fn basin_centers(&self, heightmap: &Vec<Vec<i64>>) -> Vec<(usize, usize)> {
        let height = heightmap.len();
        let width = heightmap[0].len();

        let mut basins = Vec::new();
        for i in 0..height {
            for j in 0..width {
                if i > 0 && heightmap[i - 1][j] <= heightmap[i][j] {
                    continue;
                }
                if i < height - 1 && heightmap[i + 1][j] <= heightmap[i][j] {
                    continue;
                }
                if j > 0 && heightmap[i][j - 1] <= heightmap[i][j] {
                    continue;
                }
                if j < width - 1 && heightmap[i][j + 1] <= heightmap[i][j] {
                    continue;
                }
                basins.push((i, j));
            }
        }
        basins
    }

    fn solve_actual(&self, heightmap: &Vec<Vec<i64>>) -> i64 {
        let basin_centers = self.basin_centers(heightmap);

        basin_centers
            .iter()
            .fold(0, |acc, (x, y)| acc + heightmap[*x][*y] + 1)
    }

    fn basin_bfs(&self, heightmap: &Vec<Vec<i64>>, basin_center: (usize, usize)) -> i64 {
        let height = heightmap.len();
        let width = heightmap[0].len();
        let mut seen_pos: HashSet<(usize, usize)> = HashSet::new();
        let mut to_be_checked: VecDeque<(usize, usize)> = VecDeque::new();
        to_be_checked.push_back(basin_center);

        while to_be_checked.len() > 0 {
            let (x, y) = to_be_checked.pop_front().unwrap();
            if seen_pos.contains(&(x, y)) {
                continue;
            }
            seen_pos.insert((x, y));

            if x > 0 && heightmap[x - 1][y] < 9 {
                to_be_checked.push_back((x - 1, y));
            }
            if x < height - 1 && heightmap[x + 1][y] < 9 {
                to_be_checked.push_back((x + 1, y));
            }
            if y > 0 && heightmap[x][y - 1] < 9 {
                to_be_checked.push_back((x, y - 1));
            }
            if y < width - 1 && heightmap[x][y + 1] < 9 {
                to_be_checked.push_back((x, y + 1));
            }
        }

        seen_pos.len() as i64
    }

    fn solve_actual_part2(&self, heightmap: &Vec<Vec<i64>>) -> i64 {
        let basin_centers = self.basin_centers(heightmap);
        let mut basin_sizes: Vec<i64> = basin_centers
            .iter()
            .map(|pos| self.basin_bfs(heightmap, *pos))
            .collect();

        basin_sizes.sort();
        let length = basin_sizes.len();

        basin_sizes[length - 1] * basin_sizes[length - 2] * basin_sizes[length - 3]
    }
}

impl Problem for Problem09 {
    fn name(&self) -> &str {
        "Day 9: Smoke Basin"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_09.txt");
        let heightmap = self.parse(input);
        self.solve_actual(&heightmap)
    }

    fn solve_part2(&self) -> i64 {
        let input = get_input!("./inputs/problem_09.txt");
        let heightmap = self.parse(input);
        self.solve_actual_part2(&heightmap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem09::new();
        let input = get_input!("./inputs/problem_09_example.txt");
        let heightmap = problem.parse(input);
        assert_eq!(problem.solve_actual(&heightmap), 15);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem09::new();
        let input = get_input!("./inputs/problem_09.txt");
        let heightmap = problem.parse(input);
        assert_eq!(problem.solve_actual(&heightmap), 562);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem09::new();
        let input = get_input!("./inputs/problem_09_example.txt");
        let heightmap = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&heightmap), 1134);
    }

    #[test]
    fn test_solve_actual_part2_from_input() {
        let problem = Problem09::new();
        let input = get_input!("./inputs/problem_09.txt");
        let heightmap = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&heightmap), 1076922);
    }
}
