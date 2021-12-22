use std::collections::HashMap;
fn roll(die: &mut usize, rolls: &mut usize) -> usize {
    *rolls += 1;
    let out = *die + 1;
    *die += 1;
    *die %= 3;
    out
}
fn allroll(die: &mut usize, rolls: &mut usize) -> Vec<usize> {
    let mut out = Vec::new();
    for i in 1..4 {
        for j in 1..4 {
            for k in 1..4 {
                out.push(i + j + k)
            }
        }
    }
    out
}
fn solve_a(p1: usize, p2: usize) -> usize {
    let mut universe = HashMap::new();
    universe.insert((p1 - 1, 0, p2 - 1, 0), 1);
    let mut die = 0;
    let mut rolls = 0;
    let mut p1w = 0;
    let mut p2w = 0;
    while universe.len() != 0 {
        let mut un = HashMap::new();
        for roll in allroll(&mut die, &mut rolls) {
            for (p1x, instances) in &universe {
                let pos = (p1x.0 + roll) % 10;
                let score = p1x.1 + pos + 1;
                if score >= 21 {
                    p1w += instances;
                } else {
                    *un.entry((pos, score, p1x.2, p1x.3)).or_insert(0) += instances;
                }
            }
        }
        let mut un2 = HashMap::new();
        for roll in allroll(&mut die, &mut rolls) {
            for (p1x, instances) in &un {
                let pos = (p1x.2 + roll) % 10;
                let score = p1x.3 + pos + 1;
                if score >= 21 {
                    p2w += instances;
                } else {
                    *un2.entry((p1x.0, p1x.1, pos, score)).or_insert(0) += instances;
                }
            }
        }
        universe = un2;
    };
    std::cmp::max(p1w, p2w)
}
fn main() {
    println!("{}", solve_a(7, 1));
    println!("{}", solve_a(4, 8));
}
