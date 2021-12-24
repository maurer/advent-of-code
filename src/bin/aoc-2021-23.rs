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
    fn room_id(&self) -> usize {
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

const HALL_LENGTH: usize = 11;
const ROOM_COUNT: usize = 4;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct State<const ROOM_SIZE: usize> {
    cost: usize,
    rooms: [[Option<Pod>; ROOM_SIZE]; ROOM_COUNT],
    hall: [Option<Pod>; HALL_LENGTH],
}

type Input = State<2>;

const EXTENSION: [[Pod; 2]; 4] = {
    use Pod::*;
    [[D, D], [C, B], [B, A], [A, C]]
};

fn room_id_to_hall_id(x: usize) -> usize {
    (x * 2) + 2
}

impl State<2> {
    fn parse(input: impl Iterator<Item = String>) -> Self {
        let mut rooms = [[None; 2]; 4];
        for (depth, cs) in input.skip(2).take(2).enumerate() {
            let mut room_id = 0;
            for c in cs.chars() {
                if let Some(pod) = Pod::from(c) {
                    rooms[room_id][depth] = Some(pod);
                    room_id += 1;
                }
            }
        }
        State { rooms, hall: [None; HALL_LENGTH], cost: 0 }
    }

    fn unfold_diagram(self) -> State<4> {
        let mut new_rooms = [[None; 4]; 4];
        for ((room_id, room), folded) in self.rooms.into_iter().enumerate().zip(EXTENSION.iter()) {
            new_rooms[room_id] = [room[0], Some(folded[0]), Some(folded[1]), room[1]];
        }
        State { cost: self.cost, rooms: new_rooms, hall: self.hall }
    }
}

impl<const ROOM_SIZE: usize> State<ROOM_SIZE> {
    fn normalize(&self) -> Self {
        let mut norm = *self;
        norm.cost = 0;
        norm
    }

    fn done(&self) -> bool {
        self.rooms
            .iter()
            .enumerate()
            .all(|(idx, room)| room.iter().all(|p| *p == Pod::from_room(idx)))
    }

    fn reachable(&self, hall_id: usize) -> impl Iterator<Item = usize> + '_ {
        let open = |idx: &usize| self.hall[*idx].is_none();
        (0..hall_id).rev().take_while(open).chain((hall_id + 1..HALL_LENGTH).take_while(open))
    }

    fn move_in(&self) -> Option<Self> {
        self.hall
            .iter()
            .enumerate()
            .find_map(|(hall_id, slot)| {
                slot.and_then(|pod| {
                    // If there are non-matching pods in the room or the room isn't reachable, we can't go in
                    if self.rooms[pod.room_id()].iter().any(|p| p.is_some() && p != &Some(pod))
                        || self
                            .reachable(hall_id)
                            .all(|hall_id| hall_id != room_id_to_hall_id(pod.room_id()))
                    {
                        None
                    } else {
                        Some((hall_id, pod))
                    }
                })
            })
            .map(|(hall_id, pod)| {
                let mut next_state = *self;
                next_state.hall[hall_id] = None;
                next_state.cost += pod.cost() * hall_id.abs_diff(room_id_to_hall_id(pod.room_id()));
                let back = self.rooms[pod.room_id()].iter().filter(|slot| slot.is_none()).count();
                next_state.cost += pod.cost() * back;
                next_state.rooms[pod.room_id()][back - 1] = Some(pod);
                next_state
            })
    }

    fn moves(&self) -> Vec<Self> {
        // If we can move an amphipod in, that is gauranteed optimal.
        if let Some(state) = self.move_in() {
            return vec![state];
        }

        // If we can't, check each room to see if an amphipod can move out.
        self.rooms
            .iter()
            .enumerate()
            .flat_map(|(room_id, room)| {
                let mut filled = room.iter().enumerate().skip_while(|slot| slot.1.is_none());
                if let Some((depth, Some(pod))) = filled.next() {
                    // If the room matches the end state, don't let them leave
                    if pod.room_id() == room_id && filled.all(|slot| *slot.1 == Some(*pod)) {
                        return Vec::new();
                    }
                    let mut walked_out = *self;
                    walked_out.cost += pod.cost() * (1 + depth);
                    walked_out.rooms[room_id][depth] = None;
                    self.reachable(room_id_to_hall_id(room_id))
                        .filter(|hall_id| ![2, 4, 6, 8].contains(hall_id))
                        .map(|hall_id| {
                            let mut next_state = walked_out;
                            next_state.cost += pod.cost() * room_id_to_hall_id(room_id).abs_diff(hall_id);
                            next_state.hall[hall_id] = Some(*pod);
                            next_state
                        })
                        .collect()
                } else {
                    Vec::new()
                }
            })
            .collect()
    }
}

const INPUT: &str = include_str!("../../inputs/2021/23");

fn main() {
    println!(
        "A: {}\tB: {}",
        solve_a(Input::parse(aoc::str_input(INPUT))),
        solve_b(Input::parse(aoc::str_input(INPUT)))
    );
}

fn solve<const N: usize>(initial_state: State<N>) -> usize {
    use std::cmp::Reverse;
    let mut work_queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    work_queue.push(Reverse(initial_state));
    while let Some(Reverse(state)) = work_queue.pop() {
        if state.done() {
            return state.cost;
        }
        if !visited.insert(state.normalize()) {
            continue;
        }
        for next_state in state.moves() {
            work_queue.push(Reverse(next_state));
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
