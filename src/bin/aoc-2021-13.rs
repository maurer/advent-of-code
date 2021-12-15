use sscanf::scanf;
use std::collections::{HashMap, HashSet};
struct Input {
    paper: Vec<(usize, usize)>,
    insns: Vec<(char, usize)>,
}
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/13");
fn parse(mut lines: impl Iterator<Item = String>) -> Input {
    let mut paper = Vec::new();
    while let Some(pline) = lines.next() {
        if pline == "" {
            break
        }
        let pos = scanf!(&pline, "{},{}", usize, usize).unwrap();
        paper.push(pos)
    }
    let mut insns = Vec::new();
    while let Some(iline) = lines.next() {
        let insn = scanf!(&iline, "fold along {}={}", char, usize).unwrap();
        insns.push(insn)
    }
    Input {
        paper,
        insns
    }
}
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}

fn px(v: (usize, usize)) -> usize {
    v.0
}
fn py(v: (usize, usize)) -> usize {
    v.1
}
fn ux(mut v: (usize, usize), x: usize) -> (usize, usize){
    v.0 = x;
    v
}
fn uy(mut v: (usize, usize), x: usize) -> (usize, usize){
    v.1 = x;
    v
}

fn apply((axis, idx): (char, usize), mut m: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let (proj, upd) = match axis {
        'x' => (Box::new(px) as Box<dyn Fn(_) -> usize>, Box::new(ux) as Box<dyn Fn(_, _) -> _>),
        'y' => (Box::new(py) as Box<dyn Fn(_) -> usize>, Box::new(uy) as Box<dyn Fn(_, _) -> _>),
        _ => panic!("")
    };
    let mut out = HashSet::new();
    for mut v in m.into_iter() {
        if proj(v) == idx {
            continue
        } else if proj(v) > idx {
            let new = idx - (proj(v) - idx);
            v = upd(v, new);
        }
        out.insert(v);
    }
    out
}
fn solve_a(mut input: Input) -> usize {
    let mut m = HashSet::new();
    for loc in input.paper {
        m.insert(loc);
    }
    m = apply(input.insns[0], m);
    m.len()
}
fn solve_b(mut input: Input) -> usize {
    let mut m = HashSet::new();
    for loc in input.paper {
        m.insert(loc);
    }
    for insn in input.insns {
        m = apply(insn, m);
    }
    display(m);
    0
}
fn display(m: HashSet<(usize, usize)>) {
    for y in 0..80 {
        let mut line = Vec::new();
        for x in 0..100 {
            if m.contains(&(x, y)) {
               line.push('#');
            } else {
                line.push(' ');
            }
        }
        println!("{}", line.into_iter().collect::<String>());
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 17)
    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 0)
    }
}
