type Record = Vec<usize>;
fn parse_record(line: &str) -> Record {
    let mut tokens = aoc::tokenize(line);
    let mut out = Vec::new();
    while let Some(tok) = tokens.next() {
        out.push(usize::from_str(tok).unwrap());
    }
    out
}
struct Input {
    record: Vec<Record>,
}
fn parse(mut input: impl Iterator<Item = String>) -> Input {
    let record = {
        let mut out = Vec::new();
        while let Some(line) = input.next() {
            if line.as_str() == "" {
                continue;
            }
            out.push(parse_record(&line));
        }
        out
    };
    Input { record }
}
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/7");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn solve_a(mut input: Input) -> isize {
    let x = input.record.remove(0);
    let max = *(x.iter().max().unwrap()) as isize;
    let mut min_fuel = isize::MAX;
    for k in 0..max {
        let mut fuel = 0;
        for c in &x {
            fuel += (*c as isize - k as isize).abs();
        }
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}
fn f(x: isize) -> isize {
    let mut out = 0;
    for i in 0..(x+1) {
        out += i;
    }
    out
}
fn solve_b(mut input: Input) -> isize {
    let x = input.record.remove(0);
    let max = *(x.iter().max().unwrap()) as isize;
    let mut min_fuel = isize::MAX;
    for k in 0..max {
        let mut fuel = 0;
        for c in &x {
            fuel += f((*c as isize - k as isize).abs());
        }
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    min_fuel
}
#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
16,1,2,0,4,2,7,1,2,14";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 37)
    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 168)
    }
}
