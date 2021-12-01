use crate::problem::Problem;

mod problem;
mod problem_01;

fn main() {
    println!("~ Advent of Code 2021 ~");

    let problem01 = problem_01::Problem01::new();
    problem01.solve();
}
