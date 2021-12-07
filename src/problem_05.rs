use std::collections::HashSet;

use crate::problem::Problem;

pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn as_tuple(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn get_points(&self) -> Vec<Point> {
        let mut covered_points: Vec<Point> = Vec::new();
        if self.start.x == self.end.x {
            let range = if self.start.y > self.end.y {
                self.end.y..(self.start.y + 1)
            } else {
                self.start.y..(self.end.y + 1)
            };
            range.for_each(|y| covered_points.push(Point { x: self.start.x, y }));
        } else if self.start.y == self.end.y {
            let range = if self.start.x > self.end.x {
                self.end.x..(self.start.x + 1)
            } else {
                self.start.x..(self.end.x + 1)
            };
            range.for_each(|x| covered_points.push(Point { x, y: self.start.y }));
        }
        return covered_points;
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
            let start_pt: Vec<i64> = point_strs[0]
                .split(",")
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            let end_pt: Vec<i64> = point_strs[1]
                .split(",")
                .map(|num| num.parse::<i64>().unwrap())
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
        return submarine_lines;
    }

    fn solve_actual(&self, submarine_lines: &Vec<Line>) -> i64 {
        let mut seen_once: HashSet<(i64, i64)> = HashSet::new();
        let mut seen_at_least_twice: HashSet<(i64, i64)> = HashSet::new();
        submarine_lines.iter().for_each(|line| {
            for point in line.get_points() {
                let coords = point.as_tuple();
                if seen_once.contains(&coords) {
                    seen_at_least_twice.insert(coords);
                } else {
                    seen_once.insert(coords);
                }
            }
        });
        return seen_at_least_twice.len() as i64;
    }
}

impl Problem for Problem05 {
    fn solve(&self) {
        let input = get_input!("./inputs/problem_05.txt");

        let submarine_lines = self.parse(input);

        let result = self.solve_actual(&submarine_lines);
        println!("Day 5 Answer:");
        println!(" - Part 1: {:?}", result);
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
        assert_eq!(problem.solve_actual(&submarine_lines), 5);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem05::new();
        let input = get_input!("./inputs/problem_05.txt");
        let submarine_lines = problem.parse(input);
        assert_eq!(problem.solve_actual(&submarine_lines), 4826);
    }
}
