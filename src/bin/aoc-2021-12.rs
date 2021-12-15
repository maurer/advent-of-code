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
const INPUT: &str = include_str!("../../inputs/2021/12");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn search(links: &HashMap<String, HashSet<String>>, path: Vec<String>, dup: bool) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    let last = &path[path.len() - 1];
    if last == "end" {
        return vec![path]
    }
   for next in &links[last] {
       let mut localdup = dup;
       if next == "start" {
           continue
       }
       if next.chars().next().unwrap().is_lowercase() {
           if path.contains(next) {
               if localdup {
                   continue
               } else {
                   localdup = true
               }
           }
       }
       let mut p2 = path.clone();
       p2.push(next.clone());
       out.append(&mut search(links, p2, localdup));
   }
   out
}
fn solve_a(mut input: Input) -> usize {
    let mut link = HashMap::new();
    for str_link in input.record {
        link.entry(str_link[0].clone()).or_insert_with(HashSet::new).insert(str_link[1].clone());
        link.entry(str_link[1].clone()).or_insert_with(HashSet::new).insert(str_link[0].clone());
    }
    search(&link, vec!["start".to_string()], false).len()
}
fn solve_b(mut input: Input) -> isize {
    0
}
#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 36)
    }
//    #[test]
//    fn sample_b() {
//        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 36)
//    }
}
