use sscanf::scanf;
use std::str::FromStr;

static INPUT: &str = include_str!("../../inputs/2021/5");

#[derive(Clone, Debug)]
struct Record {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl FromStr for Record {
    type Err = ();
    fn from_str(record: &str) -> Result<Self, Self::Err> {
        match scanf!(record, "{},{} -> {},{}", usize, usize, usize, usize) {
            Some((x1, y1, x2, y2)) => Ok(Record { x1, y1, x2, y2 }),
            None => Err(()),
        }
    }
}

fn parse(input: impl Iterator<Item = String>) -> impl Iterator<Item = Record> {
    input.map(|line| line.parse().unwrap())
}

fn solve_a(input: impl Iterator<Item = Record>) -> isize {
    let mut dr = Vec::new();
    dr.resize(1000, 0);
    let mut board = Vec::new();
    for _ in 0..1000 {
        board.push(dr.clone());
    }
    for mut record in input {
        if record.x1 == record.x2 {
            if record.y1 > record.y2 {
                std::mem::swap(&mut record.y2, &mut record.y1);
            }
            for y in record.y1..(record.y2 + 1) {
                board[record.x1][y] += 1;
            }
        } else if record.y1 == record.y2 {
            if record.x1 > record.x2 {
                std::mem::swap(&mut record.x2, &mut record.x1);
            }
            for x in record.x1..(record.x2 + 1) {
                board[x][record.y1] += 1;
            }
        } else {
            //panic!("diagonal: {:?}", record)
        }
    }
    let mut m: isize = 0;
    for row in board.iter() {
        for x in row {
            if *x > 1 {
                m += 1;
            }
        }
    }
    m
}
fn solve_b(input: impl Iterator<Item = Record>) -> isize {
    let mut dr = Vec::new();
    dr.resize(1000, 0);
    let mut board = Vec::new();
    for _ in 0..1000 {
        board.push(dr.clone());
    }
    for mut record in input {
        if record.x1 == record.x2 {
            if record.y1 > record.y2 {
                std::mem::swap(&mut record.y2, &mut record.y1);
            }
            for y in record.y1..(record.y2 + 1) {
                board[record.x1][y] += 1;
            }
        } else if record.y1 == record.y2 {
            if record.x1 > record.x2 {
                std::mem::swap(&mut record.x2, &mut record.x1);
            }
            for x in record.x1..(record.x2 + 1) {
                board[x][record.y1] += 1;
            }
        } else if (record.x1 as isize - record.x2 as isize).abs()
            == (record.y1 as isize - record.y2 as isize).abs()
        {
            let step_x = if record.x1 > record.x2 { -1 } else { 1 };
            let step_y = if record.y1 > record.y2 { -1 } else { 1 };
            let mut base_x = record.x1 as isize;
            let mut base_y = record.y1 as isize;
            while base_x != record.x2 as isize {
                board[base_x as usize][base_y as usize] += 1;
                base_x += step_x;
                base_y += step_y;
            }
            board[base_x as usize][base_y as usize] += 1;
            assert_eq!(base_y, record.y2 as isize);
        }
    }
    let mut m: isize = 0;
    for row in board.iter() {
        for x in row {
            if *x > 1 {
                m += 1;
            }
        }
    }
    m
}

fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;

    const TEST_INPUT: &'static str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 5)
    }

    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 12)
    }
}
