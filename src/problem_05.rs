use std::collections::{HashMap, HashSet};

use crate::problem::Problem;

pub struct Point {
    x: i64,
    y: i64,
}

pub struct Line {
    start: Point,
    end: Point,
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
        let mut seen_coords: HashMap<(i64, i64), i64> = HashMap::new();
        let mut seen_at_least_two: HashSet<(i64, i64)> = HashSet::new();
        submarine_lines.iter().for_each(|line| {
            if line.start.x == line.end.x {
                let range = if line.start.y > line.end.y {
                    line.end.y..(line.start.y + 1)
                } else {
                    line.start.y..(line.end.y + 1)
                };
                for y in range {
                    if seen_coords.contains_key(&(line.start.x, y)) {
                        (*seen_coords.get_mut(&(line.start.x, y)).unwrap()) += 1;
                        if seen_coords[&(line.start.x, y)] >= 2 {
                            seen_at_least_two.insert((line.start.x, y));
                        }
                    } else {
                        seen_coords.insert((line.start.x, y), 1);
                    }
                }
            } else if line.start.y == line.end.y {
                let range = if line.start.x > line.end.x {
                    line.end.x..(line.start.x + 1)
                } else {
                    line.start.x..(line.end.x + 1)
                };
                for x in range {
                    if seen_coords.contains_key(&(x, line.start.y)) {
                        (*seen_coords.get_mut(&(x, line.start.y)).unwrap()) += 1;
                        if seen_coords[&(x, line.start.y)] >= 2 {
                            seen_at_least_two.insert((x, line.start.y));
                        }
                    } else {
                        seen_coords.insert((x, line.start.y), 1);
                    }
                }
            }
        });
        return seen_at_least_two.len() as i64;
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
