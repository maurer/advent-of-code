type Input = Vec<Vec<char>>;
fn parse(mut input: impl Iterator<Item = String>) -> Input {
    input.map(|line| line.chars().collect_vec()).collect_vec()
}
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use sscanf::scanf;
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/25");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn get(x: &Input, i: usize, j: usize) -> char {
    let dimi = x.len();
    let dimj = x[0].len();
    x[i % dimi][j % dimj]
}
fn set(x: &mut Input, i: usize, j: usize, val: char) {
    let dimi = x.len();
    let dimj = x[0].len();
    x[i % dimi][j % dimj] = val;
}

fn do_moves(x: &Input) -> Input {
    let mut out1 = x.clone();
    for (i, row) in x.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '>' {
                if get(x, i, j + 1) == '.' {
                    set(&mut out1, i, j + 1, '>');
                    set(&mut out1, i, j, '.');
                }
            }
        }
    }
    let mut out2 = out1.clone();
    for (i, row) in x.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'v' {
                if get(&out1, i + 1, j) == '.' {
                    set(&mut out2, i, j, '.');
                    set(&mut out2, i + 1, j, 'v');
                }
            }
        }
    }
    out2
}
fn dump(x: &Input) {
    println!("STATA");
    for line in x {
        println!("{:?}", line);
    }
}
fn solve_a(mut input: Input) -> usize {
    let mut step = 0;
    loop {
        let next = do_moves(&input);
        if next == input {
            return step + 1;
        }
        input = next;
        step += 1;
    }
}
fn solve_b(mut input: Input) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 0)
    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 0)
    }
}
