use std::collections::HashMap;

use crate::problem::Problem;

pub struct Problem14 {}

impl Problem14 {
    pub fn new() -> Problem14 {
        Problem14 {}
    }

    fn parse(&self, input: String) -> (String, HashMap<String, String>) {
        let mut pair_rules: HashMap<String, String> = HashMap::new();
        let mut lines = input.lines();
        let polymer_template = lines.next().unwrap().to_string();
        lines.next();
        while let Some(rule) = lines.next() {
            let components = rule.split(" -> ").collect::<Vec<&str>>();
            pair_rules.insert(components[0].to_string(), components[1].to_string());
        }
        (polymer_template, pair_rules)
    }

    fn do_polymerization(
        &self,
        polymer_template: &String,
        pair_rules: &HashMap<String, String>,
        steps: u8,
    ) -> String {
        let mut template_clone = polymer_template.clone();
        for _ in 0..steps {
            let mut new_polymer = String::new();
            let old_as_vec = template_clone.clone().chars().collect::<Vec<char>>();
            let mut old_polymer = old_as_vec.windows(2);
            while let Some(chars) = old_polymer.next() {
                let pair = String::from_iter(chars);
                new_polymer.push(chars[0]);
                match pair_rules.contains_key(&pair) {
                    true => new_polymer.push_str(&pair_rules[&pair]),
                    false => (),
                }
            }
            new_polymer.push(old_as_vec[old_as_vec.len() - 1]);
            template_clone = new_polymer;
        }
        template_clone
    }

    fn count_chars(&self, polymer: &String) -> i64 {
        let mut char_count: HashMap<char, i64> = HashMap::new();
        polymer.chars().for_each(|c| {
            match char_count.contains_key(&c) {
                true => (*char_count.get_mut(&c).unwrap()) += 1,
                false => {
                    char_count.insert(c, 1);
                }
            };
        });

        let mut count_pairs = char_count.iter().collect::<Vec<(&char, &i64)>>();
        count_pairs.sort_by(|a, b| a.1.cmp(b.1));

        count_pairs[count_pairs.len() - 1].1 - count_pairs[0].1
    }

    fn solve_actual(&self, polymer_template: &String, pair_rules: &HashMap<String, String>) -> i64 {
        let polymer = self.do_polymerization(polymer_template, pair_rules, 10);
        self.count_chars(&polymer)
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
        (0, None)
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
}
