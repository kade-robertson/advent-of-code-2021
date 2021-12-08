#![feature(portable_simd)]

use std::time::Instant;

use crate::problem::Problem;

#[macro_use]
mod macros;
mod problem;
mod problem_01;
mod problem_02;
mod problem_03;
mod problem_04;
mod problem_05;
mod problem_06;
mod problem_07;
mod problem_08;
mod util;

fn main() {
    println!("~ Advent of Code 2021 ~");

    let problems: Vec<Box<dyn Problem>> = vec![
        Box::new(problem_01::Problem01::new()),
        Box::new(problem_02::Problem02::new()),
        Box::new(problem_03::Problem03::new()),
        Box::new(problem_04::Problem04::new()),
        Box::new(problem_05::Problem05::new()),
        Box::new(problem_06::Problem06::new()),
        Box::new(problem_07::Problem07::new()),
        Box::new(problem_08::Problem08::new()),
    ];
    problems.iter().for_each(|problem| {
        println!("{}", problem.name());
        print!(" - Part 1: ");
        let part1_start = Instant::now();
        println!("{} (took {:.2?})", problem.solve(), part1_start.elapsed());
        print!(" - Part 2: ");
        let part2_start = Instant::now();
        println!(
            "{} (took {:.2?})",
            problem.solve_part2(),
            part2_start.elapsed()
        );
    });
}
