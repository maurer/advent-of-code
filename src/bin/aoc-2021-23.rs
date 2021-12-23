#![feature(int_abs_diff)]
use std::collections::{BinaryHeap, HashSet};

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Pod {
    A,
    B,
    C,
    D,
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
            _ => None,
        }
    }
    fn room(&self) -> usize {
        use Pod::*;
        match *self {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }
    fn from_room(room: usize) -> Option<Self> {
        use Pod::*;
        match room {
            0 => Some(A),
            1 => Some(B),
            2 => Some(C),
            3 => Some(D),
            _ => None,
        }
    }
}

const HALLWAY_LENGTH: usize = 11;
const ROOM_COUNT: usize = 4;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct State<const ROOM_SIZE: usize> {
    cost: usize,
    rooms: [[Option<Pod>; ROOM_SIZE]; ROOM_COUNT],
    hallway: [Option<Pod>; HALLWAY_LENGTH],
}

type Input = State<2>;

const EXTENSION: [[Pod; 2]; 4] = {
    use Pod::*;
    [[D, D], [C, B], [B, A], [A, C]]
};

impl State<2> {
    fn parse(input: impl Iterator<Item = String>) -> Self {
        let mut rooms = [[None; 2]; 4];
        for (idx, cs) in input.skip(2).take(2).enumerate() {
            let mut room_idx = 0;
            for c in cs.chars() {
                if let Some(pod) = Pod::from(c) {
                    rooms[room_idx][idx] = Some(pod);
                    room_idx += 1;
                }
            }
        }
        State { rooms, hallway: [None; HALLWAY_LENGTH], cost: 0 }
    }

    fn unfold_diagram(self) -> State<4> {
        let mut new_rooms = [[None; 4]; 4];
        for ((rid, room), folded) in self.rooms.into_iter().enumerate().zip(EXTENSION.iter()) {
            new_rooms[rid] = [room[0], Some(folded[0]), Some(folded[1]), room[1]];
        }
        State { cost: self.cost, rooms: new_rooms, hallway: self.hallway }
    }
}

impl<const ROOM_SIZE: usize> State<ROOM_SIZE> {
    fn done(&self) -> bool {
        self.rooms
            .iter()
            .enumerate()
            .all(|(idx, room)| room.iter().all(|p| *p == Pod::from_room(idx)))
    }

    fn reachable(&self, hid: usize) -> impl Iterator<Item = usize> + '_ {
        let open = |idx: &usize| self.hallway[*idx].is_none();
        (0..hid).rev().take_while(open).chain((hid + 1..HALLWAY_LENGTH).take_while(open))
    }

    fn next(&self) -> Vec<Self> {
        let mut out = Vec::new();
        // First check any pods can move out, and to where
        'outer: for (rid, room) in self.rooms.iter().enumerate() {
            let mut ns = *self;
            let mut to_move = None;
            for (i, x) in room.iter().enumerate() {
                if let Some(pod) = x {
                    if pod.room() == rid {
                        // Check if there are pods in the back with a bad id
                        if room[i + 1..].iter().all(|p| {
                            if let Some(q) = p {
                                q.room() == rid
                            } else {
                                panic!("We shouldn' thave empty cells behind a full one")
                            }
                        }) {
                            // All the pods behind us have the right rid, don't leave
                            continue 'outer;
                        }
                    }
                    ns.cost += pod.cost() * (1 + i);
                    ns.rooms[rid][i] = None;
                    to_move = Some(*pod);
                    break;
                }
            }
            if let Some(pod) = to_move {
                for col in ns.reachable(room_to_col(rid)) {
                    if [2, 4, 6, 8].contains(&col) {
                        continue;
                    }
                    let mut nns = ns;
                    nns.cost += pod.cost() * room_to_col(rid).abs_diff(col);
                    nns.hallway[col] = Some(pod);
                    out.push(nns);
                }
            }
        }
        // Then, check if any pods can move in, and if so, to where
        for (hid, slot) in self.hallway.iter().enumerate() {
            if let Some(pod) = slot {
                if self.rooms[pod.room()].iter().any(|p| p.is_some() && p != &Some(*pod)) {
                    // We're not allowed to go in, another kind of pod is in there
                    continue;
                }
                let target = room_to_col(pod.room());
                if self.reachable(hid).any(|hid| hid == target) {
                    // We can reach our target, and there are no other pods in there, we're going
                    let mut ns = *self;
                    ns.hallway[hid] = None;
                    // Walk to the door
                    ns.cost += pod.cost() * target.abs_diff(hid);
                    assert!(self.rooms[pod.room()][0].is_none());
                    let mut back = 0;
                    for f in self.rooms[pod.room()].iter() {
                        if f.is_some() {
                            break;
                        }
                        back += 1;
                    }
                    back -= 1;
                    ns.cost += pod.cost() * (back + 1);
                    ns.rooms[pod.room()][back] = Some(*pod);
                    out.push(ns)
                }
            }
        }
        out
    }
}
fn room_to_col(x: usize) -> usize {
    (x * 2) + 2
}

const INPUT: &str = include_str!("../../inputs/2021/23");

fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(Input::parse(aoc::str_input(INPUT))),
        solve_b(Input::parse(aoc::str_input(INPUT)))
    );
}

fn solve<const N: usize>(state: State<N>) -> usize {
    use std::cmp::Reverse;
    let mut wq = BinaryHeap::new();
    let mut visited = HashSet::new();
    wq.push(Reverse(state));
    while let Some(Reverse(s)) = wq.pop() {
        if s.done() {
            return s.cost;
        }
        let mut sx = s;
        sx.cost = 0;
        if visited.contains(&sx) {
            continue;
        }
        visited.insert(sx);
        for s2 in s.next() {
            let mut z = s2;
            z.cost = 0;
            if !visited.contains(&z) {
                wq.push(Reverse(s2));
            }
        }
    }
    panic!("No solution found?");
}

fn solve_a(input: Input) -> usize {
    solve(input)
}

fn solve_b(input: Input) -> usize {
    solve(input.unfold_diagram())
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::str_input;
    const TEST_INPUT: &str = "\
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
    #[test]
    fn sample_a() {
        assert_eq!(solve_a(Input::parse(str_input(TEST_INPUT))), 12521)
    }
    #[test]
    fn sample_b() {
        assert_eq!(solve_b(Input::parse(str_input(TEST_INPUT))), 44169)
    }
}
