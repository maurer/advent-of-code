type Input = ();
fn parse(mut input: impl Iterator<Item = String>) -> Input {
    ()
}
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use sscanf::scanf;
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/24");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn solve_a(mut input: Input) -> usize {
    0
}
fn solve_b(mut input: Input) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 0)
    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 0)
    }
}
