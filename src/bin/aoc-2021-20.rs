type Input = (Vec<bool>, Vec<Vec<bool>>);
fn l2b(l: String) -> Vec<bool> {
    l.chars().map(|x| x == '#').collect_vec()
}
fn parse(mut input: impl Iterator<Item = String>) -> Input {
    let header = l2b(input.next().unwrap());
    assert_eq!("", input.next().unwrap());
    let out = input.map(l2b).collect_vec();
    (header, out)
}
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/20");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn step(i: &HashSet<(isize, isize)>, header: &[bool], sense: bool) -> (HashSet<(isize, isize)>, bool) {
    let mut out = HashSet::new();
    let outsense = if !sense && header[0] {
        !sense
    } else if sense && true {// header[0b111111111] {
        !sense
    } else {
        sense
    };
    for x in -400..400{
        for y in -400..400{
            let mut bn = 0;
            for (cx, cy) in [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y - 1), (x, y), (x, y + 1), (x + 1, y - 1), (x + 1, y), (x + 1, y + 1)].into_iter() {
                bn *= 2;
                if i.contains(&(cx, cy)) ^ sense {
                    bn += 1;
                }
            }
            if header[bn] ^ outsense {
                out.insert((x, y));
            }
        }
    }
    (out, outsense)
}
fn solve_a(mut input: Input) -> usize {
    let (header, lines) = input;
    let mut img = HashSet::new();
    for (x, line) in lines.iter().enumerate() {
        for (y, v) in line.iter().enumerate() {
            if *v {
                img.insert((x as isize, y as isize));
            }
        }
    }
    let mut sense = false;
    println!("i = {}", img.len());
    (img, sense) = step(&img, &header, sense);
    println!("1 = {}", img.len());
    (img, sense) = step(&img, &header, sense);
    println!("2 = {}", img.len());
    assert!(!sense);
    img.len()
}
fn solve_b(mut input: Input) -> usize {
    let (header, lines) = input;
    let mut img = HashSet::new();
    for (x, line) in lines.iter().enumerate() {
        for (y, v) in line.iter().enumerate() {
            if *v {
                img.insert((x as isize, y as isize));
            }
        }
    }
    let mut sense = false;
    for _ in 0..50 {
      (img, sense) = step(&img, &header, sense);
    }
    assert!(!sense);
    img.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
//    #[test]
//    fn sample_a() {
//        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 35)
//    }
//    #[test]
//    fn sample_b() {
//        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 3351)
//    }
}
