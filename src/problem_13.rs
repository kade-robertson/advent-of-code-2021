use std::collections::{HashSet, VecDeque};

use crate::problem::Problem;

enum FoldDirection {
    Horizontal,
    Vertical,
}

pub struct FoldInstruction {
    direction: FoldDirection,
    position: u16,
}
pub struct TransparentPaper {
    dots: HashSet<(u16, u16)>,
    x_max: u16,
    y_max: u16,
}

impl TransparentPaper {
    pub fn new() -> TransparentPaper {
        TransparentPaper {
            dots: HashSet::new(),
            x_max: u16::MIN,
            y_max: u16::MIN,
        }
    }

    pub fn add(&mut self, x: u16, y: u16) {
        self.dots.insert((x, y));
        self.x_max = self.x_max.max(x);
        self.y_max = self.y_max.max(y);
    }

    pub fn fold(&mut self, instruction: &FoldInstruction) {
        let mut new_dots = HashSet::new();
        for (x, y) in &self.dots {
            match instruction.direction {
                FoldDirection::Horizontal => {
                    self.y_max = instruction.position;
                    if y > &instruction.position {
                        let newy = 2 * instruction.position - *y;
                        new_dots.insert((*x, newy));
                        self.y_max = self.y_max.max(newy);
                    } else {
                        new_dots.insert((*x, *y));
                    }
                }
                FoldDirection::Vertical => {
                    self.x_max = instruction.position;
                    if x > &instruction.position {
                        let newx = 2 * instruction.position - *x;
                        new_dots.insert((newx, *y));
                        self.x_max = self.x_max.max(newx);
                    } else {
                        new_dots.insert((*x, *y));
                    }
                }
            }
        }
        self.dots = new_dots;
    }

    pub fn visible_dots(&self) -> i64 {
        self.dots.len() as i64
    }

    pub fn pretty(&self) -> String {
        let mut output: Vec<String> = vec!["           ".to_string(); self.y_max as usize];
        for row in 0..self.y_max {
            for col in 0..self.x_max {
                output[row as usize].push_str(match self.dots.contains(&(col, row)) {
                    true => "#",
                    false => ".",
                });
            }
        }
        output.join("\n")
    }
}

pub struct Problem13 {}

impl Problem13 {
    pub fn new() -> Problem13 {
        Problem13 {}
    }

    fn parse(&self, input: String) -> (TransparentPaper, VecDeque<FoldInstruction>) {
        let mut paper = TransparentPaper::new();
        let mut instructions = VecDeque::new();
        let mut lines = input.lines();

        while let Some(line) = lines.next() {
            if line.len() == 0 {
                break;
            }
            let nums = line.split(',').collect::<Vec<&str>>();
            paper.add(
                nums[0].parse::<u16>().unwrap(),
                nums[1].parse::<u16>().unwrap(),
            );
        }

        while let Some(line) = lines.next() {
            let nums = line.split('=').collect::<Vec<&str>>();
            instructions.push_back(FoldInstruction {
                direction: match nums[0].ends_with('y') {
                    true => FoldDirection::Horizontal,
                    false => FoldDirection::Vertical,
                },
                position: nums[1].parse::<u16>().unwrap(),
            });
        }

        (paper, instructions)
    }

    fn solve_actual(
        &self,
        paper: &mut TransparentPaper,
        instructions: &VecDeque<FoldInstruction>,
    ) -> i64 {
        paper.fold(&instructions[0]);
        paper.visible_dots()
    }

    fn solve_actual_part2(
        &self,
        paper: &mut TransparentPaper,
        instructions: &VecDeque<FoldInstruction>,
    ) -> i64 {
        instructions
            .iter()
            .for_each(|instruction| paper.fold(instruction));
        paper.visible_dots()
    }
}

impl Problem for Problem13 {
    fn name(&self) -> &str {
        "Day 13: Transparent Origami"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_13.txt");
        let (mut paper, instructions) = self.parse(input);
        self.solve_actual(&mut paper, &instructions)
    }

    fn solve_part2(&self) -> (i64, Option<String>) {
        let input = get_input!("./inputs/problem_13.txt");
        let (mut paper, instructions) = self.parse(input);
        (
            self.solve_actual_part2(&mut paper, &instructions),
            Some(paper.pretty()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem13::new();
        let input = get_input!("./inputs/problem_13_example.txt");
        let (mut paper, instructions) = problem.parse(input);
        assert_eq!(problem.solve_actual(&mut paper, &instructions), 17);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem13::new();
        let input = get_input!("./inputs/problem_13.txt");
        let (mut paper, instructions) = problem.parse(input);
        assert_eq!(problem.solve_actual(&mut paper, &instructions), 775);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem13::new();
        let input = get_input!("./inputs/problem_13_example.txt");
        let (mut paper, instructions) = problem.parse(input);
        problem.solve_actual_part2(&mut paper, &instructions);
        assert_eq!(
            paper.pretty().split(' ').collect::<Vec<&str>>().join(""),
            "#####\n#...#\n#...#\n#...#\n#####\n.....\n....."
        );
    }

    #[test]
    fn test_solve_actual_part2_from_input() {
        let problem = Problem13::new();
        let input = get_input!("./inputs/problem_13.txt");
        let (mut paper, instructions) = problem.parse(input);
        problem.solve_actual_part2(&mut paper, &instructions);
        assert_eq!(
            paper.pretty().split(' ').collect::<Vec<&str>>().join(""),
            "###..####.#..#.###..#..#.###..#..#.###..\n#..#.#....#..#.#..#.#..#.#..#.#.#..#..#.\n#..#.###..#..#.#..#.#..#.#..#.##...#..#.\n###..#....#..#.###..#..#.###..#.#..###..\n#.#..#....#..#.#....#..#.#....#.#..#.#..\n#..#.####..##..#.....##..#....#..#.#..#."
        );
    }
}
