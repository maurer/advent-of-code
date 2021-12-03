use sscanf::scanf;
use std::str::FromStr;

static INPUT: &'static str = include_str!("../../inputs/2021/3");

fn parse(input: impl Iterator<Item = String>) -> impl Iterator<Item = String> {
    input
}

fn solve_a(input: impl Iterator<Item = String>) -> isize {
    let mut out_1 = Vec::new();
    let mut out_2 = Vec::new();
    for line in input {
        out_1.resize(line.len(), 0);
        out_2.resize(line.len(), 0);
        for (idx, c) in line.chars().enumerate() {
            if c == '1' {
                out_2[idx] += 1;
            } else {
                out_1[idx] += 1;
            }
        }
    }
    let mut thunk = String::new();
    let mut thunk2 = String::new();
    for (x, y) in out_1.iter().zip(out_2.iter()) {
        if x > y {
            thunk.push('0');
            thunk2.push('1');
        } else {
            thunk.push('1');
            thunk2.push('0');
        }
    }
    let gamma = isize::from_str_radix(&thunk, 2).unwrap();
    let epsilon = isize::from_str_radix(&thunk2, 2).unwrap();
    gamma * epsilon
}

fn solve_b(input: impl Iterator<Item = String>) -> isize {
    let stable: Vec<_> = input.collect();
    let mut oxy_i: Vec<_> = stable.clone().into_iter().enumerate().collect();
    let mut co2_i = oxy_i.clone();
    while oxy_i.len() != 1 {
        println!("oxyi {:?}", oxy_i);
        let mut one = 0;
        let mut zero = 0;
        for (_, l) in &oxy_i {
            if l.chars().next().unwrap() == '1' {
                one += 1;
            } else {
                zero += 1;
            }
        }
        if one >= zero {
            oxy_i = oxy_i
                .into_iter()
                .filter(|(_, l)| l.chars().next().unwrap() == '1')
                .collect();
        } else {
            oxy_i = oxy_i
                .into_iter()
                .filter(|(_, l)| l.chars().next().unwrap() == '0')
                .collect();
        }
        for (_, l) in oxy_i.iter_mut() {
            let mut c = l.chars();
            c.next();
            *l = c.as_str().to_string();
        }
    }
    let oxy = isize::from_str_radix(&stable[oxy_i[0].0], 2).unwrap();
    println!("oxy {}", oxy);
    while co2_i.len() != 1 {
        let mut one = 0;
        let mut zero = 0;
        for (_, l) in &co2_i {
            if l.chars().next().unwrap() == '1' {
                one += 1;
            } else {
                zero += 1;
            }
        }
        if one < zero {
            co2_i = co2_i
                .into_iter()
                .filter(|(_, l)| l.chars().next().unwrap() == '1')
                .collect();
        } else {
            co2_i = co2_i
                .into_iter()
                .filter(|(_, l)| l.chars().next().unwrap() == '0')
                .collect();
        }
        for (_, l) in co2_i.iter_mut() {
            let mut c = l.chars();
            c.next();
            *l = c.as_str().to_string();
        }
    }
    let co2 = isize::from_str_radix(&stable[co2_i[0].0], 2).unwrap();
    println!("co2 {}", co2);
    co2 * oxy
}

fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;

    const TEST_INPUT: &'static str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 198)
    }

    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 230)
    }
}
