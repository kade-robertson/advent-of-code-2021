use std::str;

use crate::problem::Problem;

pub struct Problem03 {}

impl Problem03 {
    pub fn new() -> Problem03 {
        Problem03 {}
    }

    fn parse(&self, input: String) -> Vec<Vec<u8>> {
        input
            .lines()
            .map(|line| line.as_bytes().to_owned())
            .collect()
    }

    // Epsilon rate is just the ones complement of the gamma rate. We can
    // invert by taking the max integer value for the number of bits in the
    // input and subtracting the gamma rate.
    // ex from the problem:
    // - gamma rate is 10110
    // - max integer value with the same number of bits is 11111
    // - 11111 - 10110 = 01001
    fn epsilon_rate(&self, size: usize, num: i64) -> i64 {
        2i64.pow(size as u32) - 1 - num
    }

    fn solve_actual(&self, diagnostics: &Vec<Vec<u8>>) -> i64 {
        if diagnostics.len() == 0 {
            return 0;
        }

        let bits = diagnostics[0].len();
        let mut gamma_rate: i64 = 0;
        let mut index = 0;
        while index < bits {
            let mut ones_count = 0;
            diagnostics.iter().for_each(|num| {
                if num[index] as char == '1' {
                    ones_count += 1
                }
            });
            // Assumes we always have a least and most common bit, i.e. there
            // is never an equal amount of ones and zeroes.
            if ones_count > (diagnostics.len() / 2) {
                gamma_rate = (gamma_rate << 1) + 1;
            } else {
                gamma_rate = gamma_rate << 1;
            }
            index += 1;
        }

        gamma_rate * self.epsilon_rate(bits, gamma_rate)
    }

    fn calculate_rating(&self, values: &Vec<Vec<u8>>, invert: bool, index: usize) -> i64 {
        if values.len() == 1 {
            return i64::from_str_radix(str::from_utf8(&values[0]).unwrap(), 2).unwrap();
        }
        let mut values_to_keep: Vec<Vec<u8>> = Vec::new();
        let mut ones_count = 0;
        values.iter().for_each(|num| {
            if num[index] as char == '1' {
                ones_count += 1
            }
        });

        let halfway = values.len() / 2;
        let mut invert_bit = '1';
        let mut normal_bit = '0';
        if ones_count > halfway || ((ones_count == halfway) && (values.len() % 2 == 0)) {
            invert_bit = '0';
            normal_bit = '1';
        }

        let keep_bit = if invert { invert_bit } else { normal_bit };
        values.iter().for_each(|num| {
            if num[index] as char == keep_bit {
                values_to_keep.push(num.to_vec());
            }
        });

        self.calculate_rating(&values_to_keep, invert, index + 1)
    }

    fn solve_actual_part2(&self, diagnostics: &Vec<Vec<u8>>) -> i64 {
        if diagnostics.len() == 0 {
            return 0;
        }

        let oxygen_rating = self.calculate_rating(&diagnostics, false, 0);
        let scrubber_rating = self.calculate_rating(&diagnostics, true, 0);

        oxygen_rating * scrubber_rating
    }
}

impl Problem for Problem03 {
    fn name(&self) -> &str {
        "Day 3: Binary Diagnostic"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_03.txt");
        let diagnostics: Vec<Vec<u8>> = self.parse(input);
        self.solve_actual(&diagnostics)
    }

    fn solve_part2(&self) -> i64 {
        let input = get_input!("./inputs/problem_03.txt");
        let diagnostics: Vec<Vec<u8>> = self.parse(input);
        self.solve_actual_part2(&diagnostics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem03::new();
        let input = get_input!("./inputs/problem_03_example.txt");
        let diagnostics: Vec<Vec<u8>> = problem.parse(input);
        assert_eq!(problem.solve_actual(&diagnostics), 198);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem03::new();
        let input = get_input!("./inputs/problem_03.txt");
        let diagnostics: Vec<Vec<u8>> = problem.parse(input);
        assert_eq!(problem.solve_actual(&diagnostics), 1997414);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem03::new();
        let input = get_input!("./inputs/problem_03_example.txt");
        let diagnostics: Vec<Vec<u8>> = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&diagnostics), 230);
    }

    #[test]
    fn test_solve_actual_part2_from_input() {
        let problem = Problem03::new();
        let input = get_input!("./inputs/problem_03.txt");
        let diagnostics: Vec<Vec<u8>> = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&diagnostics), 1032597);
    }
}
