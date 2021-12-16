use itertools::Itertools;
use cached::proc_macro::cached;
use std::collections::{HashMap, HashSet, BinaryHeap};
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
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/15");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn solve_a(mut input: Input) -> usize {
    let board = input.record.into_iter().map(|s| s.chars().collect_vec()).collect_vec();
    let mut rr = Vec::new();
    rr.resize(board[0].len(), usize::MAX);
    let mut risks = Vec::new();
    risks.resize_with(board.len(), || rr.clone());
    risks[board.len() - 1][rr.len() - 1] = (board[board.len() - 1][rr.len() - 1] as usize) - ('0' as usize);
    for distance in 1..(board.len() + rr.len()) {
        let min_row = (board.len() - 1).saturating_sub(distance);
        for row in min_row..board.len() {
            let remaining_distance = distance - (board.len() - row - 1);
            let col = (rr.len() - 1).saturating_sub(remaining_distance);
            risks[row][col] = if row + 1 == risks.len() {
                risks[row][col + 1]
            } else if col + 1 == rr.len() {
                risks[row + 1][col]
            } else {
                std::cmp::min(risks[row + 1][col], risks[row][col + 1])
            } + (board[row][col] as usize - '0' as usize);
        }
    }
    risks[0][0] - c2u(board[0][0])
}
fn danger(q: &Vec<Vec<char>>, bump: usize) -> Vec<Vec<char>> {
    let mut out = Vec::new();
    for r in q {
        let mut ro = Vec::new();
        for x in r {
            ro.push((((*x as usize - '1' as usize + bump) % 9) + ('1' as usize)) as u8 as char);
        }
        out.push(ro)
    }
    out
}
fn from_tile(q: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut out = Vec::new();
    for i in 0..5 {
        for j in 0..5 {
            let tile = danger(&q, i + j);
            if q.len() * (i + 1) > out.len() {
                for r in tile {
                    out.push(r);
                }
            } else {
                for (z, r) in tile.into_iter().enumerate() {
                    out[q.len() * i + z].append(&mut r.clone());
                }
            }
        }
    }
    out
}
fn dump_board(b: &Vec<Vec<char>>) {
    for x in b {
        println!("{}", x.iter().copied().collect::<String>());
    }
}
fn c2u(x: char) -> usize {
    x as usize - '0' as usize
}
fn validate_risks(risks: &Vec<Vec<usize>>, board: &Vec<Vec<char>>) {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let down = if i + 1 < board.len() {
                risks[i + 1][j]
            } else {
                usize::MAX
            };
            let right = if j + 1 < board[0].len() {
                risks[i][j + 1]
            } else {
                usize::MAX
            };
            if (i == board.len() - 1) && (j == board[0].len() - 1) {
                continue
            }
            assert_eq!(std::cmp::min(down, right) + c2u(board[i][j]), risks[i][j]);
        }
    }
}
static mut zboard: Option<Vec<Vec<char>>> = None;

#[cached]
fn cost(x: usize, y: usize) -> usize {
    let board = unsafe {zboard.as_ref().unwrap() };
    if x == board.len() - 1 && y == board[0].len() - 1 {
        return c2u(board[x][y])
    } else if x >= board.len() || y >= board[0].len() {
        return usize::MAX
    } else {
        return std::cmp::min(cost(x + 1, y), cost(x, y + 1)) + c2u(board[x][y])
    }
}

fn solve_b(mut input: Input) -> usize {
    let tile = input.record.into_iter().map(|s| s.chars().collect_vec()).collect_vec();
    let board = from_tile(tile);
//    let mut rr = Vec::new();
//    rr.resize(board[0].len(), usize::MAX);
//    let mut risks = Vec::new();
//    risks.resize_with(board.len(), || rr.clone());
//    risks[board.len() - 1][rr.len() - 1] = (board[board.len() - 1][rr.len() - 1] as usize) - ('0' as usize);
//    for distance in 1..(board.len() + rr.len()) {
//        let min_row = (board.len() - 1).saturating_sub(distance);
//        for row in min_row..board.len() {
//            let remaining_distance = distance - (board.len() - row - 1);
//            let col = (rr.len() - 1).saturating_sub(remaining_distance);
//            risks[row][col] = if row + 1 == risks.len() {
//                risks[row][col + 1]
//            } else if col + 1 == rr.len() {
//                risks[row + 1][col]
//            } else {
//                std::cmp::min(risks[row + 1][col], risks[row][col + 1])
//            } + (board[row][col] as usize - '0' as usize);
//        }
//    }
//    validate_risks(&risks, &board);
//    risks[0][0] - c2u(board[0][0])
//      unsafe {zboard = Some(board.clone())};
//      cost(0, 0) - c2u(board[0][0])
      let mut paths: BinaryHeap<(isize, (usize, usize))> = BinaryHeap::new();
      paths.push((0, (0, 0)));
      let mut visited = HashSet::new();
      while let Some((cost, loc)) = paths.pop() {
          if loc == (board.len() - 1, board[0].len() - 1) {
              return (-cost) as usize
          }
          if visited.insert(loc) {
              for step in adj(loc, board.len(), board[0].len()) {
                  paths.push((cost - (c2u(board[step.0][step.1]) as isize), step));
              }
          }
      }
      panic!("LKJ")
}

fn adj(loc: (usize, usize), r: usize, c: usize) -> Vec<(usize, usize)>{
    let mut out = Vec::new();
    if loc.0 > 0 {
        out.push((loc.0 - 1, loc.1))
    }
    if loc.1 > 0 {
        out.push((loc.0, loc.1 - 1))
    }
    if loc.0 < r - 1 {
        out.push((loc.0 + 1, loc.1))
    }

   if loc.1 < c - 1 {
        out.push((loc.0, loc.1 + 1))
    }
   out
}



#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 40)
    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 315)
    }
}
