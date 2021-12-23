type Input = State;
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct State {
    cost: usize,
    rooms: [[Option<Pod>;2];4],
    hallway: [Option<Pod>; 11],
}
fn diff(x: usize, y: usize) -> usize {
    ((x as isize) - (y as isize)).abs() as usize
}
impl State {
    fn dump(&self) {
        for x in self.hallway.iter() {
            match x {
                Some(p) => print!("{:?}", p),
                None => print!("_"),
            }
        }
        print!("\n##");
        for x in self.rooms.iter() {
            match x[0] {
                Some(p) => print!("{:?}", p),
                None => print!("_"),
            }
            print!("#");
        }
        print!("#");
        print!("\n##");
        for x in self.rooms.iter() {
            match x[1] {
                Some(p) => print!("{:?}", p),
                None => print!("_"),
            }
            print!("#");
        }
        print!("#\nCost: {}\n", self.cost);
    }
 
    fn done(&self) -> bool {
        use Pod::*;
        self.rooms == [[Some(A), Some(A)],[Some(B), Some(B)], [Some(C), Some(C)], [Some(D), Some(D)]]
    }
    fn reachable(&self, hid: usize) -> Vec<usize> {
        let mut out = Vec::new();
        for left in (0..hid).rev() {
            if self.hallway[left].is_none() {
                out.push(left);
            } else {
                break;
            }
        }
        for right in (hid+1..11) {
            if self.hallway[right].is_none() {
                out.push(right);
            } else {
                break;
            }
        }
        out
    }
    fn next(&self) -> Vec<State> {
        println!("IN");
        self.dump();
        let mut out = Vec::new();
        // First check any pods can move out, and to where
        for (rid, room) in self.rooms.iter().enumerate() {
            let mut ns = *self;
            let mut to_move = None;
            if let Some(pod) = room[0] {
                if pod.idx() == rid {
                    // Check if there's one in the back with wrong rid, otherwise we're staying put
                    if let Some(backpod) = room[1] {
                        if backpod.idx() == rid {
                            continue;
                        }
                    } else {
                        panic!("We always move to the back, so this should never happen")
                    }
                }
                // One step to hallway
                ns.cost += pod.cost();
                ns.rooms[rid][0] = None;
                to_move = Some(pod)
            } else if let Some(pod) = room[1] {
                if pod.idx() == rid {
                    continue;
                }
                // Two steps to hallway
                ns.cost += pod.cost() * 2;
                ns.rooms[rid][1] = None;
                to_move = Some(pod);
            }
            if let Some(pod) = to_move {
                for col in ns.reachable(room_to_col(rid)) {
                    if [2, 4, 6, 8].contains(&col) {
                        continue;
                    }
                    let mut nns = ns;
                    nns.cost += pod.cost() * diff(room_to_col(rid), col);
                    nns.hallway[col] = Some(pod);
                    out.push(nns);
                }
            }
        }
        // Then, check if any pods can move in, and if so, to where
        for (hid, slot) in self.hallway.iter().enumerate() {
            if let Some(pod) = slot {
                if self.rooms[pod.idx()].iter().any(|p| p.is_some() && p != &Some(*pod)) {
                    // We're not allowed to go in, another kind of pod is in there
                    continue;
                }
                let target = room_to_col(pod.idx());
                if self.reachable(hid).contains(&target) {
                    // We can reach our target, and there are no other pods in there, we're going
                    let mut ns = *self;
                    ns.hallway[hid] = None;
                    // Walk to the door
                    ns.cost += pod.cost() * diff(target, hid);
                    assert!(self.rooms[pod.idx()][0].is_none());
                    if self.rooms[pod.idx()][1].is_none() {
                        ns.cost += pod.cost() * 2;
                        ns.rooms[pod.idx()][1] = Some(*pod);
                    } else {
                        ns.cost += pod.cost();
                        ns.rooms[pod.idx()][0] = Some(*pod);
                    }
                    out.push(ns)
                }
            }
        }
        // Mulling is illegal, this wasn't the missing move
        //// Finally, check for mulling in case that's useful
        //for (hid, slot) in self.hallway.iter().enumerate() {
        //    if let Some(pod) = slot {
        //        for c in self.reachable(hid) {
        //            if [2, 4, 6, 8].contains(&c) {
        //                continue;
        //            }
        //            let mut ns = *self;
        //            ns.cost += pod.cost() * diff(c, hid);
        //            ns.hallway[c] = Some(*pod);
        //            ns.hallway[hid] = None;
        //            out.push(ns);
        //        }
        //    }
        //}
        println!("OUT");
        for x in &out {
            x.dump();
        }
        out
    }
}
fn room_to_col(x: usize) -> usize {
    (x * 2) + 2
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Pod {
    A,
    B,
    C,
    D
}
impl Pod {
    fn cost(&self) -> usize {
        use Pod::*;
        match *self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }
    fn from(c: char) -> Option<Self> {
        use Pod::*;
        match c {
            'A' => Some(A),
            'B' => Some(B),
            'C' => Some(C),
            'D' => Some(D),
            _ => None
        }
    }
    fn idx(&self) -> usize {
        use Pod::*;
        match *self {
            A => 0,
            B => 1,
            C => 2,
            D => 3
        }
    }
}
fn parse(mut input: impl Iterator<Item = String>) -> Input {
    let mut rooms = [[None;2];4];
    for (r, cs) in input.skip(2).take(2).enumerate() {
        let mut col = 0;
        for c in cs.chars() {
            if let Some(pod) = Pod::from(c) {
                rooms[col][r] = Some(pod);
                col += 1;
            }
        }
    }
    State {
        rooms,
        hallway: [None; 11],
        cost: 0,
    }
}
use itertools::Itertools;
use std::collections::{HashMap, HashSet, BinaryHeap};
use sscanf::scanf;
use std::str::FromStr;
const INPUT: &str = include_str!("../../inputs/2021/23");
fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(parse(aoc::str_input(INPUT))),
        solve_b(parse(aoc::str_input(INPUT)))
    );
}
fn solve_a(mut input: Input) -> usize {
    use std::cmp::Reverse;
    let mut wq = BinaryHeap::new();
    let mut visited = HashSet::new();
    wq.push(Reverse(input));
    while let Some(Reverse(mut s)) = wq.pop() {
        let mut sx = s;
        sx.cost = 0;
        if visited.contains(&sx) {
            continue;
        }
        visited.insert(sx);
        for s2 in s.next() {
            if s2.done() {
                return s2.cost;
            }
            let mut z = s2;
            z.cost = 0;
            if !visited.contains(&z) {
                wq.push(Reverse(s2));
            }
        }
    }
    panic!("No solution found?");
}
fn solve_b(mut input: Input) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &'static str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(parse(str_input(TEST_INPUT))), 12521)
    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(parse(str_input(TEST_INPUT))), 0)
    }
}
