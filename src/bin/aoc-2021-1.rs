use std::str::FromStr;

fn parse(input: impl Iterator<Item = String>) -> Vec<u64> {
    input.map(|s| u64::from_str(&s).unwrap()).collect()
}

fn solve_a(input: &[u64]) -> u64 {
    solve(input, 1)
}

fn solve_b(input: &[u64]) -> u64 {
    solve(input, 3)
}

fn solve(input: &[u64], width: usize) -> u64 {
    let mut prev_sum = u64::MAX;
    let mut increase = 0;
    for window in input.windows(width) {
        let sum = window.iter().sum();
        if sum > prev_sum {
            increase += 1;
        }
        prev_sum = sum;
    }
    increase
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
199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn sample_a() {
        assert_eq!(solve_a(&parse(str_input(TEST_INPUT))), 7)
    }

    #[test]
    fn sample_b() {
        assert_eq!(solve_b(&parse(str_input(TEST_INPUT))), 5)
    }
}
