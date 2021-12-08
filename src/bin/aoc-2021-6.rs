use sscanf::scanf;
use std::str::FromStr;

static INPUT: &str = include_str!("../../inputs/2021/6");

fn parse(mut input: impl Iterator<Item = String>) -> Vec<usize> {
    input.next().unwrap().split(',').map(|x| usize::from_str(x).unwrap()).collect()
}

fn step(fish: &mut Vec<usize>) {
    let mut to_add = 0;
    for x in fish.iter_mut() {
        if *x == 0 {
            to_add += 1;
            *x = 6
        } else {
            *x -= 1;
        }
    }
    for _ in 0..to_add {
        fish.push(8)
    }
}

fn step2(fish: &mut Vec<usize>) {
    let to_add = fish.remove(0);
    fish[6] += to_add;
    fish.push(to_add);
}


fn solve_a(mut input: Vec<usize>) -> isize {
    let mut fish = Vec::new();
    fish.resize(9, 0);
    for u in input {
        fish[u] += 1;
    }
    println!("{:?}", fish);
     for _ in 0..80 {
        step2(&mut fish);
        println!("{:?}", fish);
    }
    fish.into_iter().sum::<usize>() as isize
}
fn solve_b(mut input: Vec<usize>) -> isize {
    let mut fish = Vec::new();
    fish.resize(9, 0);
    for u in input {
        fish[u] += 1;
    }
     for _ in 0..256 {
        step2(&mut fish);
    }
    fish.into_iter().sum::<usize>() as isize
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

    const TEST_INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 5934)
    }

    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 26984457539)
    }
}
