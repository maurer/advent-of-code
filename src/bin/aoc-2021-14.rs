use std::collections::HashMap;
use itertools::Itertools;
use cached::proc_macro::cached;
static mut xlat: Option<HashMap<(char, char), char>> = None;
type Header = String;
fn parse_header(line: &str) -> Header {
    let mut tokens = aoc::tokenize(line);
    let tok = tokens.next().unwrap();
    assert!(tokens.next().is_none());
    String::from(tok)
}
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
    header: Header,
    record: Vec<Record>,
}
fn parse(mut input: impl Iterator<Item = String>) -> Input {
    let header = parse_header(&input.next().unwrap());
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
    Input { header, record }
}
fn step(state: &str, map: &HashMap<(char, char), char>) -> String {
    let mut out = Vec::new();
    let mut last = '!';
    for x in state.chars() {
        if let Some(ins) =  map.get(&(last, x)) {
            out.push(*ins);
        }
        last = x;
        out.push(x);
    }
    out.into_iter().collect()
}
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/14");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn solve_a(mut input: Input) -> usize {
    let mut state = input.header;
    let mut map = HashMap::new();
    for lin in input.record {
        let x: Vec<_> = lin[0].chars().collect();
        map.insert((x[0], x[1]), lin[1].chars().next().unwrap());
    }
    for _ in 0..10 {
        state = step(&state, &map);
    }
    println!("state {}", state);
    let mut qq = Vec::new();
    for (_, x) in &state.chars().sorted().group_by(|x| *x) {
        qq.push(x.count())
    }
    qq.sort();
    qq.last().unwrap() - qq[0]
}
fn merge(mut l: HashMap<char, usize>, r: HashMap<char, usize>) -> HashMap<char, usize> {
    println!("merge l/r {:?}/{:?}", l, r);
    for (k, v) in r.into_iter() {
        let q = l.entry(k).or_insert(0);
        *q = *q + v;
    }
    l
}
fn basic(l: char, r: char) -> HashMap<char, usize> {
    let mut q = HashMap::new();
    *q.entry(l).or_insert(0) += 1;
    *q.entry(r).or_insert(0) += 1;
    q
}
#[cached]
fn count((l, r): (char, char), step: usize) -> HashMap<char, usize> {
    let ins = unsafe {xlat.as_ref().unwrap()};
    if step == 0 {
        println!("0: {} {}", l, r);
        return basic(l, r);
    }
    if let Some(x) = ins.get(&(l, r)) {
        let mut out = merge(count((l, *x), step - 1), count((*x, r), step - 1));
        out.entry(*x).and_modify(|x| *x = *x -1);
        println!("{}, {} @ {} = {:?}", l, r, step, out);
        out
    } else {
        println!("No match: {} {}", l, r);
        return basic(l, r);
    }
}
fn solve_b(mut input: Input) -> usize {
    let base = input.header;
    let mut map = HashMap::new();
    for lin in input.record {
        let x: Vec<_> = lin[0].chars().collect();
        map.insert((x[0], x[1]), lin[1].chars().next().unwrap());
    }
    unsafe {xlat = Some(map)};
    let mut out = HashMap::new();
    for (x, y) in base.chars().tuple_windows() {
        out = merge(out, count((x, y), 40));
    }
    for c in base.chars().skip(1).take(base.len() - 2) {
        out.entry(c).and_modify(|x| *x = *x -1);
    }
    println!("out {:?}", out);
    let mut max = 0;
    let mut min = usize::MAX;
    for v in out.into_values() {
        if v > max {
            max = v;
        }
        if v < min {
            min = v;
        }
    }
    max - min
}
#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
//    #[test]
//    fn sample_a() {
//        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 0)
//    }
//    #[test]
//    fn sample_b() {
//        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 0)
//    }
}
/*
 * NN @ 2
 * NCN @ 1
 * NBCCN @ 0
 */
