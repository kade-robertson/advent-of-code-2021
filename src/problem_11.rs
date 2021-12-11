use std::collections::HashSet;

use crate::problem::Problem;

pub struct Problem11 {}

impl Problem11 {
    pub fn new() -> Problem11 {
        Problem11 {}
    }

    fn parse(&self, input: String) -> Vec<Vec<u8>> {
        input
            .lines()
            .map(|line| line.chars().map(|c| (c as u8) - 48).collect())
            .collect()
    }

    fn simulate_step(&self, octopus_grid: &mut Vec<Vec<u8>>) -> i64 {
        let mut to_be_flashed: Vec<(usize, usize)> = Vec::new();

        // First, increase each energy level by 1. We can also save all the
        // octopodes (octopi?) grid positions that we have to flash later.
        for row in 0..octopus_grid.len() {
            for col in 0..octopus_grid[row].len() {
                octopus_grid[row][col] += 1;
                if octopus_grid[row][col] > 9 {
                    to_be_flashed.push((row, col));
                }
            }
        }

        let mut seen_this_turn: HashSet<(usize, usize)> = HashSet::new();
        while to_be_flashed.len() > 0 {
            let current = to_be_flashed.pop().unwrap();
            if seen_this_turn.contains(&current) {
                continue;
            }

            // Reset to 0, and bump all neighbor energy by one. If any go
            // >= 9, push them onto the stack.
            //
            // Not sure if there's a better way to handle the casting issue
            // The problem is that one of the usize values might be 0, so
            // subtracting one could cause a panic().
            seen_this_turn.insert(current);
            for row in (current.0 as i32) - 1..=(current.0 as i32) + 1 {
                for col in (current.1 as i32) - 1..=(current.1 as i32) + 1 {
                    if row >= 0
                        && (row as usize) < octopus_grid.len()
                        && col >= 0
                        && (col as usize) < octopus_grid[row as usize].len()
                    {
                        let (rowu, colu) = (row as usize, col as usize);
                        if (rowu, colu) == current {
                            continue;
                        }
                        octopus_grid[rowu][colu] += 1;
                        if octopus_grid[rowu][colu] > 9 {
                            to_be_flashed.push((rowu, colu));
                        }
                    }
                }
            }
        }

        seen_this_turn
            .iter()
            .for_each(|(row, col)| octopus_grid[*row][*col] = 0);

        seen_this_turn.len() as i64
    }

    fn solve_actual(&self, octopus_grid: &mut Vec<Vec<u8>>, steps: u8) -> i64 {
        let mut flashes = 0;
        for _step in 0..steps {
            flashes += self.simulate_step(octopus_grid);
        }
        flashes
    }

    fn solve_actual_part2(&self, octopus_grid: &mut Vec<Vec<u8>>) -> i64 {
        let mut steps = 0;
        loop {
            steps += 1;
            self.simulate_step(octopus_grid);

            if octopus_grid
                .iter()
                .all(|row| row.iter().all(|octopus| *octopus == 0))
            {
                break;
            }
        }
        steps
    }
}

impl Problem for Problem11 {
    fn name(&self) -> &str {
        "Day 11: Dumbo Octopus"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_11.txt");
        let mut octopus_grid = self.parse(input);
        self.solve_actual(&mut octopus_grid, 100)
    }

    fn solve_part2(&self) -> i64 {
        let input = get_input!("./inputs/problem_11.txt");
        let mut octopus_grid = self.parse(input);
        self.solve_actual_part2(&mut octopus_grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem11::new();
        let input = get_input!("./inputs/problem_11_example.txt");
        let mut octopus_grid = problem.parse(input);
        assert_eq!(problem.solve_actual(&mut octopus_grid, 100), 1656);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem11::new();
        let input = get_input!("./inputs/problem_11.txt");
        let mut octopus_grid = problem.parse(input);
        assert_eq!(problem.solve_actual(&mut octopus_grid, 100), 1717);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem11::new();
        let input = get_input!("./inputs/problem_11_example.txt");
        let mut octopus_grid = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&mut octopus_grid), 195);
    }

    #[test]
    fn test_solve_actual_part2_from_input() {
        let problem = Problem11::new();
        let input = get_input!("./inputs/problem_11.txt");
        let mut octopus_grid = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&mut octopus_grid), 476);
    }
}
