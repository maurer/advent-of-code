use std::collections::HashSet;
fn parse(mut input: impl Iterator<Item = String>) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    for line in input {
        let mut m = Vec::new();
        for x in line.chars() {
            m.push(x as usize - '0' as usize)
        }
        out.push(m);
    }
    out
}
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/11");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn adj(x: usize, y: usize, dim_x: usize, dim_y: usize) -> Vec<(usize, usize)> {
    let x = x as isize;
    let y = y as isize;
    let dim_x = dim_x as isize;
    let dim_y = dim_y as isize;
    let mut out = Vec::new();
    for (i, j) in [(x - 1, y - 1), (x - 1, y), (x - 1, y + 1),  (x, y - 1), (x, y + 1) , (x + 1, y - 1), (x + 1, y) , (x + 1, y + 1)] {
        if (i < 0) || (j < 0) || i >= dim_x || j >= dim_y {
            continue
        }
        out.push((i as usize, j as usize));
    }
    out
}
fn step(e: &mut Vec<Vec<usize>>) -> usize {
    for z in e.iter_mut() {
        for q in z.iter_mut() {
            *q += 1;
        }
    }
    let mut flashed = HashSet::new();
    let mut flash = true;
    let dim_x = e.len();
    let dim_y = e[0].len();
    while flash {
        flash = false;
        let mut updates: Vec<(usize, usize)> = Vec::new();
        for (x, z) in e.iter().enumerate() {
            for (y, q) in z.iter().enumerate() {
                if *q > 9 && !flashed.contains(&(x, y)) {
                    flash = true;
                    flashed.insert((x, y));
                    updates.append(&mut adj(x, y, dim_x, dim_y));
                }
            }
        }
        for (x, y) in updates {
            e[x][y] += 1
        }
    }
    let out = flashed.len();
    for (x, y) in flashed {
        e[x][y] = 0;
    }
    out
}
fn solve_a(mut input: Vec<Vec<usize>>) -> usize {
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut input);
    }
    flashes
}
fn solve_b(mut input: Vec<Vec<usize>>) -> usize {
    let all = input.len() * input[0].len();
    let mut idx = 0;
    loop {
        idx += 1;
        if step(&mut input) == all {
            return idx;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 1656)
    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 195)
    }
}
