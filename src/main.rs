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
mod problem_09;
mod problem_10;
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
        Box::new(problem_09::Problem09::new()),
        Box::new(problem_10::Problem10::new()),
    ];
    let mut duration = Instant::now().elapsed();
    problems.iter().for_each(|problem| {
        println!("{}", problem.name());

        print!(" - Part 1: ");
        let part1_start = Instant::now();
        let part1_result = problem.solve();
        let part1_duration = part1_start.elapsed();
        duration += part1_duration;
        println!("{} (took {:.2?})", part1_result, part1_duration);

        print!(" - Part 2: ");
        let part2_start = Instant::now();
        let part2_result = problem.solve_part2();
        let part2_duration = part2_start.elapsed();
        duration += part2_duration;
        println!("{} (took {:.2?})", part2_result, part2_duration);
    });
    println!("Took a total of {:.2?}", duration);
}
