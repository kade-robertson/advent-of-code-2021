use std::collections::HashSet;

use crate::problem::Problem;

pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    pub fn as_tuple(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn get_points(&self, include_diagonals: bool) -> Vec<Point> {
        let mut covered_points: Vec<Point> = Vec::new();

        let x_range: Box<dyn Iterator<Item = u16>> = if self.end.x > self.start.x {
            Box::new(self.start.x..=self.end.x)
        } else {
            Box::new((self.end.x..=self.start.x).rev())
        };

        let y_range: Box<dyn Iterator<Item = u16>> = if self.end.y > self.start.y {
            Box::new(self.start.y..=self.end.y)
        } else {
            Box::new((self.end.y..=self.start.y).rev())
        };

        if self.start.x == self.end.x {
            y_range.for_each(|y| covered_points.push(Point { x: self.start.x, y }));
        } else if self.start.y == self.end.y {
            x_range.for_each(|x| covered_points.push(Point { x, y: self.start.y }));
        } else if include_diagonals {
            x_range
                .zip(y_range)
                .for_each(|(x, y)| covered_points.push(Point { x, y }));
        }

        covered_points
    }
}

pub struct Problem05 {}

impl Problem05 {
    pub fn new() -> Problem05 {
        Problem05 {}
    }

    fn parse(&self, input: String) -> Vec<Line> {
        let mut submarine_lines: Vec<Line> = Vec::new();
        input.lines().for_each(|line| {
            let point_strs: Vec<&str> = line.split(" -> ").collect();
            let start_pt: Vec<u16> = point_strs[0]
                .split(",")
                .map(|num| num.parse::<u16>().unwrap())
                .collect();
            let end_pt: Vec<u16> = point_strs[1]
                .split(",")
                .map(|num| num.parse::<u16>().unwrap())
                .collect();
            submarine_lines.push(Line {
                start: Point {
                    x: start_pt[0],
                    y: start_pt[1],
                },
                end: Point {
                    x: end_pt[0],
                    y: end_pt[1],
                },
            });
        });
        submarine_lines
    }

    fn solve_actual(&self, submarine_lines: &Vec<Line>, include_diagonals: bool) -> u16 {
        let mut seen_once: HashSet<(u16, u16)> = HashSet::new();
        let mut seen_at_least_twice: HashSet<(u16, u16)> = HashSet::new();
        submarine_lines.iter().for_each(|line| {
            for point in line.get_points(include_diagonals) {
                let coords = point.as_tuple();
                if seen_once.contains(&coords) {
                    seen_at_least_twice.insert(coords);
                } else {
                    seen_once.insert(coords);
                }
            }
        });
        seen_at_least_twice.len() as u16
    }
}

impl Problem for Problem05 {
    fn name(&self) -> &str {
        "Day 5: Hydrothermal Venture"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_05.txt");
        let submarine_lines = self.parse(input);
        self.solve_actual(&submarine_lines, false) as i64
    }

    fn solve_part2(&self) -> i64 {
        let input = get_input!("./inputs/problem_05.txt");
        let submarine_lines = self.parse(input);
        self.solve_actual(&submarine_lines, true) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem05::new();
        let input = get_input!("./inputs/problem_05_example.txt");
        let submarine_lines = problem.parse(input);
        assert_eq!(problem.solve_actual(&submarine_lines, false), 5);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem05::new();
        let input = get_input!("./inputs/problem_05.txt");
        let submarine_lines = problem.parse(input);
        assert_eq!(problem.solve_actual(&submarine_lines, false), 4826);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem05::new();
        let input = get_input!("./inputs/problem_05_example.txt");
        let submarine_lines = problem.parse(input);
        assert_eq!(problem.solve_actual(&submarine_lines, true), 12);
    }

    #[test]
    fn test_solve_actual_part2_from_input() {
        let problem = Problem05::new();
        let input = get_input!("./inputs/problem_05.txt");
        let submarine_lines = problem.parse(input);
        assert_eq!(problem.solve_actual(&submarine_lines, true), 16793);
    }
}
