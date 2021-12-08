use std::collections::{HashMap, HashSet};
type Record = Vec<String>;
fn parse_record(line: &str) -> Record {
    let mut tokens = aoc::tokenize(line);
    let mut out = Vec::new();
    while let Some(tok) = tokens.next() {
        out.push(String::from(tok));
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
const INPUT: &str = include_str!("../../inputs/2021/8");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn solve_a(mut input: Input) -> usize {
        input.record.into_iter().map(|record| record.into_iter().skip(10).filter(|s| [2, 4, 3, 7].contains(&s.len())).count()).sum::<usize>()
}
const DIGITS: &[&str] = &["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];
fn solve_key(mut samples: Vec<String>) -> HashMap<char, char> {
    let mut must = HashMap::new();
    for good in samples.iter().filter(|s| [2, 4, 3, 7].contains(&s.len())) {
        match good.len() {
            2 => {
                for c in good.chars() {
                    let mut s = HashSet::new();
                    s.insert('c');
                    s.insert('f');
                    let mut e =must.entry(c).or_insert(s.clone());
                    *e = &*e & &s;
                }
            }
            3 => {
                for c in good.chars() {
                    let mut s = HashSet::new();
                    s.insert('a');
                    s.insert('c');
                    s.insert('f');
                    let mut e =must.entry(c).or_insert(s.clone());
                    *e = &*e & &s;
                }
            }
            4 => {
                for c in good.chars() {
                    let mut s = HashSet::new();
                    s.insert('b');
                    s.insert('c');
                    s.insert('d');
                    s.insert('f');
                    let mut e =must.entry(c).or_insert(s.clone());
                    *e = &*e & &s;
                }
            }
            _ => ()
        }
    }
    let unc: Vec<_> = samples.into_iter().filter(|s| ![2, 4, 3, 7].contains(&s.len())).collect();
    search(unc, must, vec![1, 4, 7, 8]).unwrap().into_iter().map(|(k, v)| {(k, v.into_iter().next().unwrap())}).collect()
}
fn search(mut unc: Vec<String>, must: HashMap<char, HashSet<char>>, mut chosen: Vec<usize>) -> Option<HashMap<char, HashSet<char>>> {
    if unc.len() == 0 {
        return Some(must)
    }
    println!("must: {:?}", must);
    let ee = unc.pop().unwrap();
    let choices: Vec<(usize, &'static &'static str)> = DIGITS.iter().enumerate().filter(|(_, s)| s.len() == ee.len()).filter(|(d, _)| !chosen.contains(d)).collect();
    'outer: for (choice, s) in choices {
        let ms: HashSet<char> = s.chars().collect();
        let mut m2 = must.clone();
        for c in ee.chars() {
            let mut e = m2.entry(c).or_insert(ms.clone());
            *e = &*e & &ms;
            if e.len() == 0 {
                continue 'outer
            }
        }
        let mut uniq: Vec<char> = Vec::new();
        for v in m2.values() {
            if v.len() == 1 {
                let c: char = *v.iter().next().unwrap();
                if uniq.contains(&c) {
                    break;
                }
                uniq.push(c);
            }
        }
        for v in m2.values_mut() {
            if v.len() != 1 {
                for c in &uniq {
                    v.remove(c);
                }
            }
        }
        let mut c2 = chosen.clone();
        c2.push(choice);
        println!("Descending, choice={}", choice);
        if let Some(x) = search(unc.clone(), m2, c2) {
            println!("Succ");
            return Some(x)
        }
        println!("Failure, continuing");
    }
    println!("No choices, up");
    None
}
fn render(key: &HashMap<char, char>, val: &[String]) -> usize  {
    let mut out = 0;
    for digit in val {
        if digit == "|" {
            continue;
        }
        out *= 10;
        let mut on = Vec::new();
        for l in digit.chars() {
            if l == '|' {
                continue;
            }
            on.push(key[&l])
        }
        on.sort();
        let s: String = on.into_iter().collect();
        let mut dk = 999999;
        for (n, digit) in DIGITS.iter().enumerate() {
            if *digit == &s {
                dk = n;
                break;
            }
        }
        out += dk;
    }
    out
}
fn solve_b(mut input: Input) -> usize {
    let mut out = 0;
    for record in input.record {
        let sample: Vec<String> = record.clone().into_iter().take(10).collect();
        let digits: Vec<String> = record.into_iter().skip(10).collect();
        let key = solve_key(sample);
        let num = render(&key, &digits);
        out += num;
    }
    out
}
#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    const SMALL_TEST_INPUT: &'static str = "\
acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 26)
    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 61229)
    }
    #[test]
    fn small_b() {
        assert_eq!(solve_b(parse(str_input(SMALL_TEST_INPUT))), 61229)
    }
}
