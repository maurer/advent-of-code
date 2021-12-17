use bitvec::prelude::*;
type BV = BitVec<Msb0, u8>;
type BS = BitSlice<Msb0, u8>;
type Record = String;
fn parse_record(line: &str) -> Record {
    let mut tokens = aoc::tokenize(line);
    let tok = tokens.next().unwrap();
    assert!(tokens.next().is_none());
    String::from(tok)
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
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/16");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn line_to_bitvec(x: &str) -> BV {
    let mut out = BV::new();
    for mut cs in &x.chars().chunks(2) {
        let y = cs.next().unwrap();
        let z = cs.next().unwrap();
        let s = u8::from_str_radix(&format!("{}{}", y, z), 16).unwrap();
        let mut bv: BV = BV::from_slice(&[s]).unwrap();
        out.append(&mut bv);
    }
    out
}
#[derive(Debug)]
enum Packet {
    Literal{version: usize, lit: u64},
    Op {version: usize, op_id: u8, args: Vec<Packet>},
}
fn parse_literal(upg: u64, x: &BS) -> (u64, &BS) {
    let local: u64 = x[1..5].load_be();
    if x[0] {
        parse_literal(upg * 16 + local, &x[5..])
    } else {
        (upg * 16 + local, &x[5..])
    }
}
fn score_packet(p: &Packet) -> usize {
    match *p {
        Packet::Literal { ref version, .. } => *version,
        Packet::Op { ref version, ref args, .. } => *version + args.iter().map(score_packet).sum::<usize>()
    }
}
fn compute_packet(p: &Packet) -> isize {
    match *p {
        Packet::Literal { ref lit, .. } => *lit as isize,
        Packet::Op {ref op_id, ref args, ..} => {
            let mut qq = args.iter().map(compute_packet);
            match *op_id {
                0 => qq.sum::<isize>(),
                1 => qq.product::<isize>(),
                2 => qq.min().unwrap(),
                3 => qq.max().unwrap(),
                5 => if qq.next().unwrap() > qq.next().unwrap() {
                    1
                } else {
                    0
                }
                6 => if qq.next().unwrap() < qq.next().unwrap() {
                    1
                } else {
                    0
                }
                7 => if qq.next().unwrap() == qq.next().unwrap() {
                    1
                } else {
                    0
                }
                _ => panic!("badop: {:?}", op_id),
            }
        }
    }
}
fn parse_packet(x: &BS) -> (Packet, &BS) {
    let version: usize = x[0..3].load_be();
    let type_id: u8 = x[3..6].load_be();
    match type_id {
        4 => {
            let (lit, rest) = parse_literal(0, &x[6..]);
            (Packet::Literal{version, lit}, rest)
        }
        _ => {
            if !x[6] {
                //15-bit length of remaining packets
                let subpack_bitlen: usize = x[7..22].load_be();
                let mut rem = &x[22..(22 + subpack_bitlen)];
                let mut subs = Vec::new();
                while rem.len() != 0 {
                    let (packet, rp) = parse_packet(rem);
                    subs.push(packet);
                    rem = rp;
                }
                (Packet::Op {version, op_id: type_id, args: subs }, &x[(22 + subpack_bitlen)..])
            } else {
                //11-bit packet count
                let subpack_count: usize = x[7..18].load_be();
                let mut rem = &x[18..];
                let mut subs = Vec::new();
                for _ in 0..subpack_count {
                    let (packet, rp) = parse_packet(rem);
                    subs.push(packet);
                    rem = rp;
                }
                (Packet::Op {version, op_id: type_id, args: subs }, rem)
            }
        }
    }
}
fn solve_a(mut input: Input) -> usize {
    let mut bv = BV::new();
    for line in input.record {
        bv.append(&mut line_to_bitvec(&line));
    }
    let (packet, rem) = parse_packet(&bv);
    score_packet(&packet)
}
fn solve_b(mut input: Input) -> isize {
    let mut bv = BV::new();
    for line in input.record {
        bv.append(&mut line_to_bitvec(&line));
    }
    let (packet, rem) = parse_packet(&bv);
    compute_packet(&packet)
}
#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
8A004A801A8002F478";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 16)
    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 15)
   }
}
