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

    fn solve_actual(&self, heightmap: &Vec<Vec<i64>>) -> i64 {
        let height = heightmap.len();
        let width = heightmap[0].len();

        let mut total = 0;
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
                total += heightmap[i][j] + 1;
            }
        }
        return total;
    }
}

impl Problem for Problem09 {
    fn name(&self) -> &str {
        "Day 9: Smoke Basin"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_09.txt");
        let heightmap = self.parse(input);
        return self.solve_actual(&heightmap);
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
        let problem = Problem09::new();
        let input = get_input!("./inputs/problem_09_example.txt");
        let heightmap = problem.parse(input);
        assert_eq!(problem.solve_actual(&heightmap), 15);
    }
}
