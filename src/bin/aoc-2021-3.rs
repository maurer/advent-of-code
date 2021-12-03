static INPUT: &str = include_str!("../../inputs/2021/3");

fn parse(input: impl Iterator<Item = String>) -> impl Iterator<Item = Vec<bool>> {
    input.map(|line| line.chars().map(|c| c == '1').collect())
}

fn to_bias(bit: bool) -> isize {
    if bit {
        1
    } else {
        -1
    }
}

fn bits_to_usize(bits: impl Iterator<Item = bool>) -> usize {
    let mut out = 0;
    for bit in bits {
        out <<= 1;
        if bit {
            out += 1;
        }
    }
    out
}

fn solve_a(mut input: impl Iterator<Item = Vec<bool>>) -> usize {
    let mut biases: Vec<_> = input.next().unwrap().into_iter().map(to_bias).collect();
    for bits in input {
        for (bias, bit_bias) in biases.iter_mut().zip(bits.into_iter()) {
            *bias += to_bias(bit_bias)
        }
    }

    let gamma = bits_to_usize(biases.iter().map(|bias| *bias > 0));
    let epsilon = bits_to_usize(biases.iter().map(|bias| *bias <= 0));

    gamma * epsilon
}

fn winnow(mut input: Vec<Vec<bool>>, sense: bool) -> Vec<bool> {
    for idx in 0..input[0].len() {
        if input.len() <= 1 {
            break;
        }
        let bias: isize = input.iter().map(|bits| to_bias(bits[idx])).sum();
        input = input
            .into_iter()
            .filter(|bits| bits[idx] == (bias >= 0) ^ sense)
            .collect();
    }
    input.remove(0)
}

fn solve_b(input: impl Iterator<Item = Vec<bool>>) -> usize {
    let stable: Vec<_> = input.collect();
    let oxy = bits_to_usize(winnow(stable.clone(), false).into_iter());
    let co2 = bits_to_usize(winnow(stable, true).into_iter());
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
