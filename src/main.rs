use crate::problem::Problem;

mod problem;
mod problem_01;

fn main() {
    println!("~ Advent of Code 2021 ~");
    problem_01::Problem01::solve();
}
