use crate::problem::Problem;

pub struct SignalNote {
    signal_patterns: Vec<String>,
    output: Vec<String>,
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
                .map(|pattern| pattern.to_string())
                .collect(),
        }
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
}

impl Problem for Problem08 {
    fn name(&self) -> &str {
        "Day 7: The Treachery of Whales"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_08.txt");
        let signal_notes = self.parse(input);
        return self.solve_actual(&signal_notes);
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
}
