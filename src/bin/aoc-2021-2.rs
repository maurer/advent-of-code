use sscanf::scanf;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Dir {
    Forward,
    Down,
    Up,
}

impl FromStr for Dir {
    type Err = ();
    fn from_str(dir: &str) -> Result<Self, Self::Err> {
        match dir {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
struct Action {
    dir: Dir,
    magnitude: isize,
}

impl FromStr for Action {
    type Err = ();
    fn from_str(action: &str) -> Result<Self, Self::Err> {
        match scanf!(action, "{} {}", String, isize) {
            Some((dir_str, magnitude)) => Ok(Action {
                dir: Dir::from_str(&dir_str)?,
                magnitude,
            }),
            None => Err(()),
        }
    }
}

fn parse(input: impl Iterator<Item = String>) -> impl Iterator<Item = Action> {
    input.map(|line| line.parse().unwrap())
}

fn solve_a(input: impl Iterator<Item = Action>) -> isize {
    let mut horizontal: isize = 0;
    let mut depth: isize = 0;
    for action in input {
        match action.dir {
            Dir::Down => depth += action.magnitude,
            Dir::Up => depth -= action.magnitude,
            Dir::Forward => horizontal += action.magnitude,
        }
    }
    horizontal * depth
}

fn solve_b(input: impl Iterator<Item = Action>) -> isize {
    let mut horizontal: isize = 0;
    let mut depth: isize = 0;
    let mut aim: isize = 0;
    for action in input {
        match action.dir {
            Dir::Down => aim += action.magnitude,
            Dir::Up => aim -= action.magnitude,
            Dir::Forward => {
                horizontal += action.magnitude;
                depth += aim * action.magnitude;
            }
        }
    }
    horizontal * depth
}

fn main() {
    // In order to run both solutions, we need to collect the iterator to allow two executions. If
    // we were running on a large file such that memory or disk IO was an issue, we could run a
    // single solution without collecting, and IO would be interleaved appropriately.
    let input: Vec<_> = parse(aoc::stdin_input()).collect();
    println!(
        "A: {}\tB: {}",
        solve_a(input.iter().copied()),
        solve_b(input.iter().copied())
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;

    const TEST_INPUT: &'static str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 150)
    }

    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 900)
    }
}
