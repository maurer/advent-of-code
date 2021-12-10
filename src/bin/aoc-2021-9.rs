use std::collections::HashMap;
type Record = String;
fn parse_record(line: &str) -> Vec<usize> {
    let mut out = Vec::new();
    for c in line.chars() {
        out.push(c as usize - '0' as usize);
    }
    out
}
struct Input {
    record: Vec<Vec<usize>>,
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
const INPUT: &str = include_str!("../../inputs/2021/9");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn adjacent(board: &Vec<Vec<usize>>, row: isize, col: isize) -> Vec<(isize, isize)> {
    let mut out = Vec::new();
    for (x, y) in [(row - 1, col), (row, col - 1), (row + 1, col), (row, col +1)] {
        if x >= 0 && board.len() as isize > x {
            let row = &board[x as usize];
            if y >= 0 && row.len() as isize > y {
                out.push((x, y));
            }
        }
    }
    out
}
fn solve_a(mut input: Input) -> isize {
    0
    //let mut risk: isize = 0;
    //for x in 0..(input.record.len() as isize) {
    //    for y in 0..(input.record[0].len() as isize) {
    //        println!("{}, {}", x, y);
    //        let adj = adjacent(&input.record, x, y);
    //        println!("{:?}", adj);
    //        if adj.into_iter().all(|k| input.record[x as usize][y as usize] < k) {
    //            println!("yes");
    //            risk += input.record[x as usize][y as usize] as isize + 1;
    //        }
    //    }
    //}
    //risk
}
fn solve_b(mut input: Input) -> usize {
    let mut basins = HashMap::new();
    let mut b2 = HashMap::new();
    for x in 0..(input.record.len() as isize) {
        for y in 0..(input.record[0].len() as isize) {
            let adj = adjacent(&input.record, x, y);
            if adj.into_iter().all(|k| input.record[x as usize][y as usize] < input.record[k.0 as usize][k.1 as usize]) {
                basins.insert((x, y), vec![(x, y)]);
                b2.insert((x, y), (x, y));
            }
        }
    }
    let mut changed = true;
    while changed {
        changed = false;
        for x in 0..(input.record.len() as isize) {
            for y in 0..(input.record[0].len() as isize) {
                let k =  input.record[x as usize][y as usize];
                if k == 9 {
                    continue;
                }
                if b2.contains_key(&(x, y)) {
                    continue;
                }
                let adj = adjacent(&input.record, x, y);
                for q in adj {
                    if b2.contains_key(&q) {
                        let b = b2[&q];
                        b2.insert((x, y), b);
                        basins.get_mut(&b).unwrap().push((x, y));
                        changed = true;
                        break;
                    }
                }
            }
        }
    }
    let mut x: Vec<_> = basins.values().map(|b| b.len()).collect();
    x.sort();
    x.reverse();
    x[0] * x[1] * x[2]
}
#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";
//    #[test]
//    fn sample_a() {
//        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 5)
//    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 1134)
    }
}
