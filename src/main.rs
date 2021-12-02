use crate::problem::Problem;

mod problem;
mod problem_01;
mod problem_02;
mod util;
fn main() {
    println!("~ Advent of Code 2021 ~");

    let problems: Vec<Box<dyn Problem>> = vec![
        Box::new(problem_01::Problem01::new()),
        Box::new(problem_02::Problem02::new()),
    ];
    problems.iter().for_each(|problem| problem.solve());
}
