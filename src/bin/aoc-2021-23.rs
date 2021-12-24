#![feature(int_abs_diff)]
use std::collections::{BinaryHeap, HashSet};

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum Amphipod {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}
use Amphipod::*;

impl Amphipod {
    const VALUES: &'static [Amphipod] = &[A, B, C, D];
    fn cost(&self) -> usize {
        [1, 10, 100, 1000][*self as usize]
    }
    fn from(c: char) -> Option<Self> {
        (c as i64 - 'A' as i64).try_into().ok().and_then(|idx: usize| Self::VALUES.get(idx).copied())
    }
    fn room_id(&self) -> usize {
        *self as usize
    }
    fn from_room(room: usize) -> Option<Self> {
        Self::VALUES.get(room).copied()
    }
}

const HALL_LENGTH: usize = 11;
const ROOM_COUNT: usize = 4;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct State<const ROOM_SIZE: usize> {
    rooms: [[Option<Amphipod>; ROOM_SIZE]; ROOM_COUNT],
    hall: [Option<Amphipod>; HALL_LENGTH],
}

type Input = State<2>;

const EXTENSION: [[Amphipod; 2]; 4] = [[D, D], [C, B], [B, A], [A, C]];

fn room_id_to_hall_id(x: usize) -> usize {
    (x * 2) + 2
}

impl State<2> {
    fn parse(input: impl Iterator<Item = String>) -> Self {
        let mut rooms = [[None; 2]; 4];
        for (depth, cs) in input.skip(2).take(2).enumerate() {
            let mut room_id = 0;
            for c in cs.chars() {
                if let Some(pod) = Amphipod::from(c) {
                    rooms[room_id][depth] = Some(pod);
                    room_id += 1;
                }
            }
        }
        State { rooms, hall: [None; HALL_LENGTH] }
    }

    fn unfold_diagram(self) -> State<4> {
        let mut new_rooms = [[None; 4]; 4];
        for ((room_id, room), folded) in self.rooms.into_iter().enumerate().zip(EXTENSION.iter()) {
            new_rooms[room_id] = [room[0], Some(folded[0]), Some(folded[1]), room[1]];
        }
        State { rooms: new_rooms, hall: self.hall }
    }
}

impl<const ROOM_SIZE: usize> State<ROOM_SIZE> {
    fn done(&self) -> bool {
        self.rooms
            .iter()
            .enumerate()
            .all(|(idx, room)| room.iter().all(|p| *p == Amphipod::from_room(idx)))
    }

    fn reachable(&self, hall_id: usize) -> impl Iterator<Item = usize> + '_ {
        let open = |idx: &usize| self.hall[*idx].is_none();
        (0..hall_id).rev().take_while(open).chain((hall_id + 1..HALL_LENGTH).take_while(open))
    }

    fn move_in(&self) -> Option<(usize, Self)> {
        self.hall
            .iter()
            .enumerate()
            .find_map(|(hall_id, slot)| {
                slot.and_then(|pod| {
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
                let back = self.rooms[pod.room_id()].iter().filter(|slot| slot.is_none()).count();
                let cost = pod.cost() * (back + hall_id.abs_diff(room_id_to_hall_id(pod.room_id())));
                next_state.rooms[pod.room_id()][back - 1] = Some(pod);
                (cost, next_state)
            })
    }

    fn moves(&self) -> impl Iterator<Item = (usize, Self)> + '_ {
        let move_in = self.move_in().map(|new_state| [new_state].into_iter());
        let move_out = if move_in.is_none() {
            Some(self.rooms.iter().enumerate().flat_map(move |(room_id, room)| {
                let mut filled = room.iter().enumerate().skip_while(|slot| slot.1.is_none());
                let ff = filled.next().map(|(x, y)| (x, *y));
                ff.into_iter()
                    .filter_map(move |(depth, slot)| {
                        slot.and_then(|pod| {
                            if pod.room_id() == room_id && filled.all(|slot| *slot.1 == Some(pod)) {
                                return None;
                            }
                            Some((depth, pod))
                        })
                    })
                    .flat_map(move |(depth, pod)| {
                        let mut walked_out = *self;
                        walked_out.rooms[room_id][depth] = None;
                        self.reachable(room_id_to_hall_id(room_id))
                            .filter(|hall_id| ![2, 4, 6, 8].contains(hall_id))
                            .map(move |hall_id| {
                                let mut next_state = walked_out;
                                let cost = pod.cost() * (1 + depth + room_id_to_hall_id(room_id).abs_diff(hall_id));
                                next_state.hall[hall_id] = Some(pod);
                                (cost, next_state)
                            })
                    })
            }))
        } else {
            None
        };
        move_in.into_iter().flatten().chain(move_out.into_iter().flatten())
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
    work_queue.push(Reverse((0, initial_state)));
    while let Some(Reverse((cost, state))) = work_queue.pop() {
        if state.done() {
            return cost;
        }
        if !visited.insert(state) {
            continue;
        }
        for (delta_cost, next_state) in state.moves() {
            work_queue.push(Reverse((cost + delta_cost, next_state)));
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
