use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use itertools::Itertools;
use ndarray::array;
use ndarray::Array1;
use ndarray::Array2;
use ndarray::ArrayBase;
use ndarray::Axis;
use ndarray::Data;
use ndarray::Dimension;
use ndarray::RawData;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    W,
    H,
    X, // Hallways in front of rooms.
    R(char),
}

use Tile::H;
use Tile::R;
use Tile::W;
use Tile::X;

impl Tile {
    fn to_char(&self) -> char {
        match self {
            W => '#',
            H => '.',
            X => ',',
            R(c) => c.to_ascii_lowercase(),
        }
    }
}

fn load_map1() -> Array2<Tile> {
    array![
        [W, W, W, W, W, W, W, W, W, W, W, W, W],
        [W, H, H, X, H, X, H, X, H, X, H, H, W],
        [W, W, W, R('A'), W, R('B'), W, R('C'), W, R('D'), W, W, W],
        [H, H, W, R('A'), W, R('B'), W, R('C'), W, R('D'), W, H, H],
        [H, H, W, W, W, W, W, W, W, W, W, H, H]
    ]
}
fn load_map2() -> Array2<Tile> {
    array![
        [W, W, W, W, W, W, W, W, W, W, W, W, W],
        [W, H, H, X, H, X, H, X, H, X, H, H, W],
        [W, W, W, R('A'), W, R('B'), W, R('C'), W, R('D'), W, W, W],
        [H, H, W, R('A'), W, R('B'), W, R('C'), W, R('D'), W, H, H],
        [H, H, W, R('A'), W, R('B'), W, R('C'), W, R('D'), W, H, H],
        [H, H, W, R('A'), W, R('B'), W, R('C'), W, R('D'), W, H, H],
        [H, H, W, W, W, W, W, W, W, W, W, H, H]
    ]
}
fn initial_state1() -> State {
    State {
        hallway: Array1::from_elem((7,), None),
        rooms: array![
            [Some('A'), Some('C'), Some('B'), Some('B')],
            [Some('D'), Some('D'), Some('A'), Some('C')]
        ],
    }
}
fn initial_state2() -> State {
    State {
        hallway: Array1::from_elem((7,), None),
        rooms: array![
            [Some('A'), Some('C'), Some('B'), Some('B')],
            [Some('D'), Some('C'), Some('B'), Some('A')],
            [Some('D'), Some('B'), Some('A'), Some('C')],
            [Some('D'), Some('D'), Some('A'), Some('C')]
        ],
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    hallway: Array1<Option<char>>,
    rooms: Array2<Option<char>>,
}

fn dumb_array_compare<S: Data, D: Dimension>(
    arr: &ArrayBase<S, D>,
    other: &ArrayBase<S, D>,
) -> Ordering
where
    <S as RawData>::Elem: Ord,
{
    arr.iter()
        .zip(other.iter())
        .map(|(h, o)| h.cmp(o))
        .find(|ord| ord != &Ordering::Equal)
        .unwrap_or(Ordering::Equal)
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match dumb_array_compare(&self.hallway, &other.hallway) {
            Ordering::Equal => {}
            ord => return Some(ord),
        };
        return Some(dumb_array_compare(&self.rooms, &other.rooms));
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
fn move_cost(c: char) -> i64 {
    match c {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!(),
    }
}

#[derive(Clone, Copy)]
enum Coord {
    Hallway(usize),
    Room([usize; 2]),
}

impl Coord {
    fn to_grid(&self) -> [usize; 2] {
        match self {
            Coord::Hallway(i) => [1, [1, 2, 4, 6, 8, 10, 11][*i]],
            Coord::Room(pos) => [2 + pos[0], 3 + pos[1] * 2],
        }
    }
    fn distance(&self, other: &Self) -> i64 {
        let vertical_cost: usize = [self, other]
            .iter()
            .map(|x| match x {
                Coord::Hallway(_) => 0,
                Coord::Room(pos) => pos[0] + 1,
            })
            .sum();
        let horizontal_cost = self.to_grid()[1].abs_diff(other.to_grid()[1]);
        return (vertical_cost + horizontal_cost) as i64;
    }
    fn deref<'a>(&self, state: &'a mut State) -> &'a mut Option<char> {
        match self {
            Coord::Hallway(i) => &mut state.hallway[*i],
            Coord::Room(pos) => &mut state.rooms[*pos],
        }
    }
}

fn desired_room(c: char) -> usize {
    (c as u8 - 'A' as u8) as usize
}

impl State {
    fn display(&self, map: &Array2<Tile>) {
        let mut a = map.mapv(|t| t.to_char());
        for (i, occupant) in self.hallway.indexed_iter() {
            if let Some(o) = occupant {
                a[Coord::Hallway(i).to_grid()] = *o;
            }
        }
        for (ind, occupant) in self.rooms.indexed_iter() {
            if let Some(o) = occupant {
                a[Coord::Room([ind.0, ind.1]).to_grid()] = *o;
            }
        }
        for row in 0..a.nrows() {
            for col in 0..a.ncols() {
                print!("{}", a[[row, col]]);
            }
            println!();
        }
    }

    fn solved(&self) -> bool {
        if !self.hallway.iter().all(|o| o == &None) {
            return false;
        }
        for ((_row, col), occupant) in self.rooms.indexed_iter() {
            if let Some(c) = occupant {
                if desired_room(*c) != col {
                    return false;
                }
            }
        }
        return true;
    }

    fn hallway_clear(&self, start: Coord, dest: Coord) -> bool {
        let start_col = start.to_grid()[1];
        let end_col = dest.to_grid()[1];
        let bounds = if start_col < end_col {
            start_col + 1..=end_col
        } else {
            end_col..=start_col - 1
        };
        return self.hallway.indexed_iter().all(|(i, occupant)| {
            if bounds.contains(&Coord::Hallway(i).to_grid()[1]) {
                occupant == &None
            } else {
                true
            }
        });
    }

    fn children(&self) -> Vec<(State, i64)> {
        let mut result = Vec::new();
        let top_of_room = self
            .rooms
            .axis_iter(Axis(1))
            .map(|a| {
                a.into_iter()
                    .find_position(|c| **c != None)
                    .map(|(i, o)| (i, o.unwrap()))
            })
            .collect_vec();
        let is_destination = self
            .rooms
            .axis_iter(Axis(1))
            .enumerate()
            .map(|(i, room)| {
                room.iter().all(|occupant| match occupant {
                    Some(c) => desired_room(*c) == i,
                    None => true,
                })
            })
            .collect_vec();

        let mut try_path = |start: Coord, end: Coord, name: char| {
            if self.hallway_clear(start, end) {
                let mut new_state = self.clone();
                *start.deref(&mut new_state) = None;
                *end.deref(&mut new_state) = Some(name);
                let step = start.distance(&end);
                result.push((new_state, step * move_cost(name)));
            }
        };
        for (i, actor) in self.hallway.indexed_iter() {
            let start_pos = Coord::Hallway(i);
            if let Some(name) = actor {
                let home = desired_room(*name);
                if is_destination[home] {
                    let new_pos = Coord::Room([
                        top_of_room[home].map(|x| x.0).unwrap_or(self.rooms.nrows()) - 1,
                        home,
                    ]);
                    try_path(start_pos, new_pos, *name);
                }
            }
        }
        for (room, t) in top_of_room.iter().enumerate() {
            if let Some((depth, name)) = t {
                let home = desired_room(*name);
                if room == home && is_destination[home] {
                    continue;
                }
                let starting_position = Coord::Room([*depth, room]);
                if is_destination[home] {
                    let new_position = Coord::Room([
                        top_of_room[home].map(|x| x.0).unwrap_or(self.rooms.nrows()) - 1,
                        home,
                    ]);
                    try_path(starting_position, new_position, *name);
                }
                for (i, occupant) in self.hallway.indexed_iter() {
                    if occupant == &None {
                        try_path(starting_position, Coord::Hallway(i), *name);
                    }
                }
            }
        }
        return result;
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node(i64, State);

fn djikstras(map: &Array2<Tile>, initial_state: State) -> Option<i64> {
    let mut visited = HashMap::<State, i64>::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(Node(0, initial_state)));
    let mut count = 0;
    while let Some(Reverse(Node(cost, state))) = queue.pop() {
        count += 1;
        if count % 1000 == 0 {
            println!("Counted {} states, up to cost {}", count, cost);
        }

        if state.solved() {
            println!("Solved! {}", cost);
            state.display(&map);
            return Some(cost);
        }
        if visited.get(&state).map(|&v| v < cost).unwrap_or(false) {
            continue;
        }

        for (child, step_cost) in state.children() {
            let child_cost = cost + step_cost;
            let update = match visited.get(&child) {
                Some(last) => last > &child_cost,
                None => true,
            };
            if update {
                visited.insert(child.clone(), child_cost);
                queue.push(Reverse(Node(child_cost, child)));
            }
        }
    }
    return None;
}
#[test]
fn part1() {
    let map = load_map1();
    let state = initial_state1();
    let cost = djikstras(&map, state);
    println!("cost {:?}", cost);
}

#[test]
fn part2() {
    let map = load_map2();
    let state = initial_state2();
    let cost = djikstras(&map, state);
    println!("cost {:?}", cost);
}
