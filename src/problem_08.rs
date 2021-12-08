use std::collections::HashMap;

use crate::problem::Problem;

pub struct SignalNote {
    signal_patterns: Vec<String>,
    output: Vec<String>,
}

fn pattern_to_bits(pattern: &String) -> u8 {
    let mut value = 0u8;
    pattern
        .chars()
        .for_each(|c| value += 1 << (c as u32 - 'a' as u32));
    return value;
}

impl SignalNote {
    pub fn new(input: &str) -> SignalNote {
        let input_string = Box::new(input.to_string());
        let split_note: Vec<&str> = input_string.split(" | ").collect();
        SignalNote {
            signal_patterns: split_note[0]
                .split(' ')
                .map(|pattern| pattern.to_string())
                .collect(),
            output: split_note[1]
                .split(' ')
                .map(|number| number.to_string())
                .collect(),
        }
    }

    pub fn determine_patterns(&self) -> HashMap<u8, i64> {
        let mut known_values: HashMap<i64, u8> = HashMap::new();
        let mut known_bits: HashMap<&String, u8> = HashMap::new();

        // Get the easy ones out of the way.
        self.signal_patterns.iter().for_each(|pattern| {
            let bit_pattern = pattern_to_bits(pattern);
            known_bits.insert(pattern, bit_pattern);
            match bit_pattern.count_ones() {
                2 => known_values.insert(1, bit_pattern),
                3 => known_values.insert(7, bit_pattern),
                4 => known_values.insert(4, bit_pattern),
                7 => known_values.insert(8, bit_pattern),
                _ => None,
            };
        });

        self.signal_patterns.iter().for_each(|pattern| {
            let bit_pattern = known_bits[pattern];

            // 0, 6 and 9 are the only patterns with 6 signals.
            // 9 and 0 includes all of 1s bits, 6 does not, so bitwise AND with 1
            // should determine if it is a (9 or 0) or a 6.
            // Then, only 9 includes all of 4s bits, so a bitwise AND with 4 should
            // determine if it is a 9 or a 0.
            if pattern.len() == 6 {
                known_values.insert(
                    if (bit_pattern & known_values[&1]).count_ones() == 2 {
                        if (bit_pattern & known_values[&4]).count_ones() == 4 {
                            9
                        } else {
                            0
                        }
                    } else {
                        6
                    },
                    bit_pattern,
                );
            } else if pattern.len() == 5 {
                // 2, 3 and 5 are the only patterns with 5 signals.
                // 3 and 5 includes all but 1 of 4s bits, but 2 is missing 2 bits.
                // (num | 4) == 8 should determine if it is a 2 or a (3 or 5).
                // Then, only 3 includes all of 1s bits, so a bitwise AND with 1 should
                // determine if it is a 3 or a 5
                known_values.insert(
                    if (bit_pattern & known_values[&4]).count_ones() == 3 {
                        if (bit_pattern & known_values[&1]).count_ones() == 2 {
                            3
                        } else {
                            5
                        }
                    } else {
                        2
                    },
                    bit_pattern,
                );
            }
        });

        let mut solved_values: HashMap<u8, i64> = HashMap::new();
        known_values.iter().for_each(|(value, pattern)| {
            solved_values.insert(*pattern, *value);
        });
        return solved_values;
    }

    pub fn get_value(&self) -> i64 {
        let solved_values = self.determine_patterns();
        let mut value = 0;
        self.output
            .iter()
            .for_each(|num| value = value * 10 + (solved_values[&pattern_to_bits(num)]));
        return value;
    }
}

pub struct Problem08 {}

impl Problem08 {
    pub fn new() -> Problem08 {
        Problem08 {}
    }

    fn parse(&self, input: String) -> Vec<SignalNote> {
        input.lines().map(|line| SignalNote::new(line)).collect()
    }

    fn solve_actual(&self, signal_notes: &Vec<SignalNote>) -> i64 {
        let mut total_easy_digits = 0;
        signal_notes.iter().for_each(|note| {
            note.output.iter().for_each(|digit| match digit.len() {
                2 | 3 | 4 | 7 => total_easy_digits += 1,
                _ => (),
            })
        });
        return total_easy_digits;
    }

    fn solve_actual_part2(&self, signal_notes: &Vec<SignalNote>) -> i64 {
        return signal_notes
            .iter()
            .fold(0, |total, note| total + note.get_value());
    }
}

impl Problem for Problem08 {
    fn name(&self) -> &str {
        "Day 8: Seven Segment Search"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_08.txt");
        let signal_notes = self.parse(input);
        return self.solve_actual(&signal_notes);
    }

    fn solve_part2(&self) -> i64 {
        let input = get_input!("./inputs/problem_08.txt");
        let signal_notes = self.parse(input);
        return self.solve_actual_part2(&signal_notes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_to_bits() {
        assert_eq!(pattern_to_bits(&"a".to_string()), 1);
        assert_eq!(pattern_to_bits(&"b".to_string()), 2);
        assert_eq!(pattern_to_bits(&"c".to_string()), 4);
        assert_eq!(pattern_to_bits(&"d".to_string()), 8);
        assert_eq!(pattern_to_bits(&"e".to_string()), 16);
        assert_eq!(pattern_to_bits(&"f".to_string()), 32);
        assert_eq!(pattern_to_bits(&"g".to_string()), 64);
        assert_eq!(pattern_to_bits(&"abcdefg".to_string()), 127);
        assert_eq!(pattern_to_bits(&"aeg".to_string()), 81);
    }

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem08::new();
        let input = get_input!("./inputs/problem_08_example.txt");
        let signal_notes = problem.parse(input);
        assert_eq!(problem.solve_actual(&signal_notes), 26);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem08::new();
        let input = get_input!("./inputs/problem_08.txt");
        let signal_notes = problem.parse(input);
        assert_eq!(problem.solve_actual(&signal_notes), 245);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem08::new();
        let input = get_input!("./inputs/problem_08_example.txt");
        let signal_notes = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&signal_notes), 61229);
    }

    #[test]
    fn test_solve_actual_part2_from_input() {
        let problem = Problem08::new();
        let input = get_input!("./inputs/problem_08.txt");
        let signal_notes = problem.parse(input);
        assert_eq!(problem.solve_actual_part2(&signal_notes), 983026);
    }
}
