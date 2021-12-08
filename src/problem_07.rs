use crate::problem::Problem;

pub struct Problem07 {}

impl Problem07 {
    pub fn new() -> Problem07 {
        Problem07 {}
    }

    fn parse(&self, input: String) -> Vec<i64> {
        input
            .split(',')
            .map(|line| line.parse::<i64>().unwrap())
            .collect()
    }

    fn solve_actual(&self, crab_submarines: &Vec<i64>) -> i64 {
        let mut positions = crab_submarines.to_owned();
        positions.sort();
        let ideal_position = positions[positions.len() / 2];
        return positions
            .iter()
            .fold(0, |acc, crab| acc + (crab - ideal_position).abs());
    }
}

impl Problem for Problem07 {
    fn name(&self) -> &str {
        "Day 7: The Treachery of Whales"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_07.txt");
        let crab_submarines = self.parse(input);
        return self.solve_actual(&crab_submarines);
    }

    fn solve_part2(&self) -> i64 {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem07::new();
        let input = get_input!("./inputs/problem_07_example.txt");
        let crab_submarines = problem.parse(input);
        assert_eq!(problem.solve_actual(&crab_submarines), 37);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem07::new();
        let input = get_input!("./inputs/problem_07.txt");
        let crab_submarines = problem.parse(input);
        assert_eq!(problem.solve_actual(&crab_submarines), 342641);
    }
}
