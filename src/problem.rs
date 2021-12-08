pub trait Problem {
    fn name(&self) -> &str;
    fn solve(&self) -> i64;
    fn solve_part2(&self) -> i64;
}
