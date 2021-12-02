use std::str::FromStr;
use sscanf::scanf;

enum Dir {
    Forward,
    Down,
    Up
}

impl Dir {
    fn from_str(dir: &str) -> Dir {
        use Dir::*;
        match dir {
            "forward" => Forward,
            "down" => Down,
            "up" => Up,
            _ => panic!("bad dir")
        }
    }
}

struct Action {
    dir: Dir,
    magnitude: isize
}

fn parse_record(line: String) -> Action {
    let (dir_str, magnitude) = scanf!(line, "{} {}", String, isize).unwrap();
    Action {
        dir: Dir::from_str(&dir_str),
        magnitude
    }
}

fn parse(input: impl Iterator<Item = String>) -> Vec<Action> {
    input.map(parse_record).collect()
}

fn solve_a(input: &[Action]) -> isize {
    use Dir::*;
    let mut h: isize = 0;
    let mut d: isize = 0;
    for action in input {
        match action.dir {
            Down => d += action.magnitude,
            Up => d -= action.magnitude,
            Forward => h += action.magnitude,
        }
    }
    h * d
}

fn solve_b(input: &[Action]) -> isize {
    use Dir::*;
    let mut h: isize = 0;
    let mut d: isize = 0;
    let mut aim: isize = 0;
    for action in input {
        match action.dir {
            Down => aim += action.magnitude,
            Up => aim -= action.magnitude,
            Forward => {
                h += action.magnitude;
                d += aim * action.magnitude;
            }
        }
    }
    h * d
}

fn main() {
    let input = parse(aoc::stdin_input());
    println!("A: {}\tB: {}", solve_a(&input), solve_b(&input));
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
        assert_eq!(solve_a(&parse(str_input(TEST_INPUT))), 150)
    }

    #[test]
    fn sample_b() {
        assert_eq!(solve_b(&parse(str_input(TEST_INPUT))), 900)
    }
}
