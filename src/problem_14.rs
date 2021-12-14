use std::collections::HashMap;

use crate::problem::Problem;

/// The largest string we'd expect to see is 2 characters, and each character
/// can only be one of 26 values, so at most needs 5 bits to be represented.
/// Similar to problem 12, we encode all strings as u16s, shifting by 5 bits
/// to keep all relevant info.
fn str_to_num(value: &str) -> u16 {
    value
        .chars()
        .fold(0, |acc, c| (acc << 5) + (c as u16) - ('A' as u16))
}

const BITMASK: u16 = (1 << 5) - 1;

pub trait EasyUpdate<K, V> {
    fn update(&mut self, key: K, value: V);
}

impl<K: Eq + core::hash::Hash, V: std::ops::AddAssign> EasyUpdate<K, V> for HashMap<K, V> {
    fn update(&mut self, k: K, v: V) {
        if self.contains_key(&k) {
            (*self.get_mut(&k).unwrap()) += v;
        } else {
            self.insert(k, v);
        }
    }
}

pub struct Problem14 {}

impl Problem14 {
    pub fn new() -> Problem14 {
        Problem14 {}
    }

    fn parse(&self, input: String) -> (String, HashMap<u16, u16>) {
        let mut pair_rules: HashMap<u16, u16> = HashMap::new();
        let mut lines = input.lines();
        let polymer_template = lines.next().unwrap().to_string();
        lines.next();
        while let Some(rule) = lines.next() {
            let components = rule.split(" -> ").collect::<Vec<&str>>();
            pair_rules.insert(str_to_num(components[0]), str_to_num(components[1]));
        }
        (polymer_template, pair_rules)
    }

    fn build_char_map(
        &self,
        polymer_template: &String,
        pair_rules: &HashMap<u16, u16>,
    ) -> HashMap<u16, i64> {
        let mut char_count = HashMap::new();

        pair_rules.keys().for_each(|k| {
            char_count.insert(k & BITMASK, 0);
            if k > &26 {
                char_count.insert((k >> 5) & BITMASK, 0);
            }
        });

        pair_rules.values().for_each(|c| {
            char_count.insert(*c, 0);
        });

        polymer_template
            .chars()
            .for_each(|c| char_count.update(str_to_num(c.to_string().as_str()), 1));

        char_count
    }

    fn build_polymer_pairs(&self, polymer_template: &String) -> HashMap<u16, i64> {
        let mut pair_map = HashMap::new();
        let polymer_chars = polymer_template.chars().collect::<Vec<char>>();
        let mut window = polymer_chars.windows(2);
        while let Some(chars) = window.next() {
            let pair = str_to_num(chars.iter().collect::<String>().as_str());
            pair_map.update(pair, 1);
        }
        pair_map
    }

    fn do_polymerization(
        &self,
        polymer_template: &String,
        pair_rules: &HashMap<u16, u16>,
        steps: u8,
    ) -> i64 {
        let mut char_count = self.build_char_map(polymer_template, pair_rules);
        let mut polymer_pairs = self.build_polymer_pairs(polymer_template);

        for _ in 0..steps {
            let mut new_polymer_pairs = polymer_pairs.clone();
            for (pair, amount) in polymer_pairs {
                if pair_rules.contains_key(&pair) {
                    // If our pair exists in the map of rules, then we construct
                    // two new pairs. ex: NN -> C rule matches, the two new pairs
                    // are NC and CN.
                    let new_pair_1 = ((pair >> 5) << 5) + pair_rules[&pair];
                    let new_pair_2 = (pair_rules[&pair] << 5) + (pair & BITMASK);

                    // Since the rule only adds one new character, update it's count
                    // to the amount of pairs that matched this rule.
                    char_count.update(pair_rules[&pair], amount);

                    // After the replacement, the two new pairs have the same count
                    // as the original pair, and the original pair's count is
                    // reduced to 0 as it has been entirely replaced.
                    new_polymer_pairs.update(new_pair_1, amount);
                    new_polymer_pairs.update(new_pair_2, amount);
                    new_polymer_pairs.update(pair, -amount);
                }
            }
            polymer_pairs = new_polymer_pairs;
        }

        char_count.values().max().unwrap() - char_count.values().min().unwrap()
    }

    fn solve_actual(&self, polymer_template: &String, pair_rules: &HashMap<u16, u16>) -> i64 {
        self.do_polymerization(polymer_template, pair_rules, 10)
    }

    fn solve_actual_part2(&self, polymer_template: &String, pair_rules: &HashMap<u16, u16>) -> i64 {
        self.do_polymerization(polymer_template, pair_rules, 40)
    }
}

impl Problem for Problem14 {
    fn name(&self) -> &str {
        "Day 14: Extended Polymerization"
    }

    fn solve(&self) -> i64 {
        let input = get_input!("./inputs/problem_14.txt");
        let (polymer_template, pair_rules) = self.parse(input);
        self.solve_actual(&polymer_template, &pair_rules)
    }

    fn solve_part2(&self) -> (i64, Option<String>) {
        let input = get_input!("./inputs/problem_14.txt");
        let (polymer_template, pair_rules) = self.parse(input);
        (
            self.solve_actual_part2(&polymer_template, &pair_rules),
            None,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem14::new();
        let input = get_input!("./inputs/problem_14_example.txt");
        let (polymer_template, pair_rules) = problem.parse(input);
        assert_eq!(problem.solve_actual(&polymer_template, &pair_rules), 1588);
    }

    #[test]
    fn test_solve_actual_from_input() {
        let problem = Problem14::new();
        let input = get_input!("./inputs/problem_14.txt");
        let (polymer_template, pair_rules) = problem.parse(input);
        assert_eq!(problem.solve_actual(&polymer_template, &pair_rules), 2745);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem14::new();
        let input = get_input!("./inputs/problem_14_example.txt");
        let (polymer_template, pair_rules) = problem.parse(input);
        assert_eq!(
            problem.solve_actual_part2(&polymer_template, &pair_rules),
            2188189693529
        );
    }

    #[test]
    fn test_solve_actual_part2_from_input() {
        let problem = Problem14::new();
        let input = get_input!("./inputs/problem_14.txt");
        let (polymer_template, pair_rules) = problem.parse(input);
        assert_eq!(
            problem.solve_actual_part2(&polymer_template, &pair_rules),
            3420801168962
        );
    }
}
