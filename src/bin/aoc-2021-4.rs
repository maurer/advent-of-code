use std::str::FromStr;

static INPUT: &str = include_str!("../../inputs/2021/4");

#[derive(Debug, Clone)]
struct Game {
    nums: Vec<usize>,
    tables: Vec<Vec<Vec<usize>>>,
}

fn parse(mut input: impl Iterator<Item = String>) -> Game {
    let nums: Vec<_> =
        input.next().unwrap().split(",").map(|x| usize::from_str(x).unwrap()).collect();
    let mut table = Vec::new();
    let mut tables = Vec::new();
    while let Some(line) = input.next() {
        if line == "" {
            if table.len() != 0 {
                tables.push(table);
            }
            table = Vec::new();
        } else {
            table.push(
                line.split(" ").filter(|s| s != &"").map(|x| usize::from_str(x).unwrap()).collect(),
            );
        }
    }
    tables.push(table);
    Game { nums, tables }
}
fn transpose(board: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    (0..board[0].len()).map(|i| board.iter().map(|inner| inner[i]).collect()).collect()
}

fn score(board: &Vec<Vec<usize>>, nums: &[usize]) -> Option<usize> {
    let mut marked: Vec<Vec<bool>> = board
        .clone()
        .into_iter()
        .map(|row| row.into_iter().map(|num| nums.contains(&num)).collect())
        .collect();
    let mut win = false;
    if marked.iter().any(|row| row.iter().all(|x| *x)) {
        win = true;
    }
    marked = transpose(marked);
    if marked.iter().any(|row: &Vec<bool>| row.iter().all(|x| *x)) {
        win = true;
    }
    if win {
        let mut score = 0;
        for row in board {
            for num in row {
                if !nums.contains(&num) {
                    score += num;
                }
            }
        }
        Some(score * nums.last().unwrap())
    } else {
        None
    }
}

fn solve_a(input: Game) -> usize {
    for i in 0..input.nums.len() {
        for board in &input.tables {
            if let Some(score) = score(&board, &input.nums[0..i]) {
                return score;
            }
        }
    }
    panic!("at the disco")
}

fn solve_b(mut input: Game) -> usize {
    for i in 0..input.nums.len() {
        if input.tables.len() == 1 {
            if let Some(score) = score(&input.tables[0], &input.nums[0..i]) {
                return score;
            }
        }
        input.tables = input
            .tables
            .into_iter()
            .filter(|board| score(&board, &input.nums[0..i]).is_none())
            .collect();
    }
    panic!("at the disco")
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
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 4512)
    }

    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 1924)
    }
}
