fn parse(mut input: impl Iterator<Item = String>) -> impl Iterator<Item = String>{
    input
}
static OPENS: &[char] = &['{', '[', '(', '<'];
static CLOSES: &[char] = &['}', ']', ')', '>'];
fn corrupt_char(line: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if let Some(idx) = OPENS.iter().position(|x| x == &c) {
            stack.push(idx);
        } else {
            if let Some(idx) = stack.pop() {
                if CLOSES[idx] != c {
                    return Some(c);
                }
            } else {
                return Some(c);
            }
        }
    }
    None
}
fn completion(line: &str) -> Option<String> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if let Some(idx) = OPENS.iter().position(|x| x == &c) {
            stack.push(idx);
        } else {
            if let Some(idx) = stack.pop() {
                if CLOSES[idx] != c {
                    return None
                }
            } else {
                return None
            }
        }
    }
    Some(stack.into_iter().map(|x| CLOSES[x]).rev().collect())
}
fn cs(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("lkjl")
    }
}
fn score_str(comp:&str) -> usize {
    let mut out = 0;
    for c in comp.chars() {
        out *= 5;
        out += cs(c);
    }
    out
}

fn char_val(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("un"),
    }
}


use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/10");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn solve_a(mut input: impl Iterator<Item = String>) -> usize {
    let mut sum = 0;
    for line in input {
        if let Some(c) = corrupt_char(&line) {
            sum += char_val(c);
        }
    }
    sum
}
fn solve_b(mut input: impl Iterator<Item = String>) -> usize {
    let mut out = Vec::new();
    for line in input {
        if let Some(c) = completion(&line) {
            out.push(score_str(&c));
        }
    }
    out.sort();
    out[out.len() / 2]
}
#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 26397)
    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 288957)
    }
}
