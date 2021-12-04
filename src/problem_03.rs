use std::str;

use crate::problem::Problem;

pub struct Problem03 {}

impl Problem03 {
    pub fn new() -> Problem03 {
        Problem03 {}
    }

    // Epsilon rate is just the ones complement of the gamma rate. We can
    // invert by taking the max integer value for the number of bits in the
    // input and subtracting the gamma rate.
    // ex from the problem:
    // - gamma rate is 10110
    // - max integer value with the same number of bits is 11111
    // - 11111 - 10110 = 01001
    fn epsilon_rate(&self, size: usize, num: i64) -> i64 {
        let max_value = 2i64.pow(size as u32) - 1;
        return max_value - num;
    }

    fn solve_actual(&self, diagnostics: &Vec<&[u8]>) -> i64 {
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
        return gamma_rate * self.epsilon_rate(bits, gamma_rate);
    }

    fn calculate_rating(&self, values: &Vec<&[u8]>, invert: bool, index: usize) -> i64 {
        if values.len() == 1 {
            return i64::from_str_radix(str::from_utf8(values[0]).unwrap(), 2).unwrap();
        }
        let mut values_to_keep: Vec<&[u8]> = Vec::new();
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
                values_to_keep.push(num);
            }
        });
        return self.calculate_rating(&values_to_keep, invert, index + 1);
    }

    fn solve_actual_part2(&self, diagnostics: &Vec<&[u8]>) -> i64 {
        if diagnostics.len() == 0 {
            return 0;
        }

        let oxygen_rating = self.calculate_rating(&diagnostics, false, 0);
        let scrubber_rating = self.calculate_rating(&diagnostics, true, 0);

        return oxygen_rating * scrubber_rating;
    }
}

impl Problem for Problem03 {
    fn solve(&self) {
        let input = get_input!("./inputs/problem_03.txt");

        let diagnostics: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

        let result = self.solve_actual(&diagnostics);
        let result_part2 = self.solve_actual_part2(&diagnostics);
        println!("Day 3 Answer:");
        println!(" - Part 1: {}", result);
        println!(" - Part 2: {}", result_part2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem03::new();
        let diagnostics = vec![
            "00100".as_bytes(),
            "11110".as_bytes(),
            "10110".as_bytes(),
            "10111".as_bytes(),
            "10101".as_bytes(),
            "01111".as_bytes(),
            "00111".as_bytes(),
            "11100".as_bytes(),
            "10000".as_bytes(),
            "11001".as_bytes(),
            "00010".as_bytes(),
            "01010".as_bytes(),
        ];
        assert_eq!(problem.solve_actual(&diagnostics), 198);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem03::new();
        let diagnostics = vec![
            "00100".as_bytes(),
            "11110".as_bytes(),
            "10110".as_bytes(),
            "10111".as_bytes(),
            "10101".as_bytes(),
            "01111".as_bytes(),
            "00111".as_bytes(),
            "11100".as_bytes(),
            "10000".as_bytes(),
            "11001".as_bytes(),
            "00010".as_bytes(),
            "01010".as_bytes(),
        ];
        assert_eq!(problem.solve_actual_part2(&diagnostics), 230);
    }
}
