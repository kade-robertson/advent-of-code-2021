use crate::problem::Problem;

pub struct Problem06 {}

impl Problem06 {
    pub fn new() -> Problem06 {
        Problem06 {}
    }

    fn parse(&self, input: String) -> Vec<i64> {
        input
            .split(',')
            .map(|line| line.parse::<i64>().unwrap())
            .collect()
    }

    fn solve_actual(&self, initial_fish: &Vec<i64>, simulation_days: i64) -> i64 {
        let mut fish_buckets: Vec<i64> = vec![0; 9];
        initial_fish
            .iter()
            .for_each(|fish| fish_buckets[*fish as usize] += 1);

        for _ in 0..simulation_days {
            let actual_new_fish = fish_buckets[0];
            fish_buckets[0] = fish_buckets[1];
            fish_buckets[1] = fish_buckets[2];
            fish_buckets[2] = fish_buckets[3];
            fish_buckets[3] = fish_buckets[4];
            fish_buckets[4] = fish_buckets[5];
            fish_buckets[5] = fish_buckets[6];
            fish_buckets[6] = fish_buckets[7] + actual_new_fish;
            fish_buckets[7] = fish_buckets[8];
            fish_buckets[8] = actual_new_fish;
        }

        fish_buckets.iter().sum()
    }
}

impl Problem for Problem06 {
    fn name(&self) -> &str {
        "Day 6: Lanternfish"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_06.txt");
        let initial_fish = self.parse(input);
        self.solve_actual(&initial_fish, 80)
    }

    fn solve_part2(&self) -> (i64, Option<String>) {
        let input = get_input!("./inputs/problem_06.txt");
        let initial_fish = self.parse(input);
        (self.solve_actual(&initial_fish, 256), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem06::new();
        let input = get_input!("./inputs/problem_06_example.txt");
        let initial_fish = problem.parse(input);
        assert_eq!(problem.solve_actual(&initial_fish, 18), 26);
        assert_eq!(problem.solve_actual(&initial_fish, 80), 5934);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem06::new();
        let input = get_input!("./inputs/problem_06.txt");
        let initial_fish = problem.parse(input);
        assert_eq!(problem.solve_actual(&initial_fish, 80), 350917);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem06::new();
        let input = get_input!("./inputs/problem_06_example.txt");
        let initial_fish = problem.parse(input);
        assert_eq!(problem.solve_actual(&initial_fish, 256), 26984457539);
    }

    #[test]
    fn test_solve_actual_part2_from_input() {
        let problem = Problem06::new();
        let input = get_input!("./inputs/problem_06.txt");
        let initial_fish = problem.parse(input);
        assert_eq!(problem.solve_actual(&initial_fish, 256), 1592918715629);
    }
}
