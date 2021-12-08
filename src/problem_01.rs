use crate::problem::Problem;

pub struct Problem01 {}

impl Problem01 {
    pub fn new() -> Problem01 {
        Problem01 {}
    }

    fn parse(&self, input: String) -> Vec<i64> {
        input
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect()
    }

    fn solve_actual(&self, measurements: &Vec<i64>) -> i64 {
        let mut increases = 0;
        let mut window = measurements.windows(2);
        while let Some([prev, next]) = window.next() {
            if next > prev {
                increases += 1;
            }
        }
        return increases;
    }

    fn solve_actual_part2(&self, measurements: &Vec<i64>) -> i64 {
        let mut increases = 0;
        let mut window = measurements.windows(4);
        while let Some([first, second, third, fourth]) = window.next() {
            if (second + third + fourth) > (first + second + third) {
                increases += 1;
            }
        }
        return increases;
    }
}

impl Problem for Problem01 {
    fn name(&self) -> &str {
        "Day 1: Sonar Sweep"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_01.txt");
        let measurements = self.parse(input);
        return self.solve_actual(&measurements);
    }

    fn solve_part2(&self) -> i64 {
        let input = get_input!("./inputs/problem_01.txt");
        let measurements = self.parse(input);
        return self.solve_actual_part2(&measurements);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem01::new();
        let measurements: Vec<i64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(problem.solve_actual(&measurements), 7);
    }

    #[test]
    fn test_solve_actual_always_increasing() {
        let problem = Problem01::new();
        let measurements: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(problem.solve_actual(&measurements), 6);
    }

    #[test]
    fn test_solve_actual_always_decreasing() {
        let problem = Problem01::new();
        let measurements: Vec<i64> = vec![7, 6, 5, 4, 3, 2, 1];
        assert_eq!(problem.solve_actual(&measurements), 0);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem01::new();
        let measurements: Vec<i64> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(problem.solve_actual_part2(&measurements), 5);
    }

    #[test]
    fn test_solve_actual_part2_always_increasing() {
        let problem = Problem01::new();
        let measurements: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(problem.solve_actual_part2(&measurements), 4);
    }

    #[test]
    fn test_solve_actual_part2_always_decreasing() {
        let problem = Problem01::new();
        let measurements: Vec<i64> = vec![7, 6, 5, 4, 3, 2, 1];
        assert_eq!(problem.solve_actual_part2(&measurements), 0);
    }
}
