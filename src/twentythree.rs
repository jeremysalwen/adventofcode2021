use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;
use ndarray::array;
use ndarray::Array;
use ndarray::Array2;

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

fn load_map() -> Array2<Tile> {
    array![
        [W, W, W, W, W, W, W, W, W, W, W, W, W],
        [W, H, H, X, H, X, H, X, H, X, H, H, H],
        [W, W, W, R('A'), W, R('B'), W, R('C'), W, R('D'), W, W, W],
        [H, H, W, R('A'), W, R('B'), W, R('C'), W, R('D'), W, H, H],
        [H, H, W, W, W, W, W, W, W, W, W, H, H]
    ]
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Actor {
    name: char,
    position: [i8; 2],
    has_stopped_in_hall: bool,
}

impl Actor {
    fn new(name: char, position: [i8; 2]) -> Actor {
        Actor {
            name,
            position,
            has_stopped_in_hall: false,
        }
    }
    fn idx(&self) -> [usize; 2] {
        self.position.map(|x| x as usize)
    }
    fn move_cost(&self) -> i64 {
        match self.name {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!(),
        }
    }
    fn desired_column(&self) -> i8 {
        match self.name {
            'A' => 3,
            'B' => 5,
            'C' => 7,
            'D' => 9,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct State {
    actors: Vec<Actor>,
    last_moved: Option<usize>,
}

impl State {
    fn display(&self, map: &Array2<Tile>) {
        let mut a = map.mapv(|t| t.to_char());
        for actor in &self.actors {
            a[actor.idx()] = actor.name;
        }
        println!("Last moved {:?}", self.last_moved);
        for row in 0..a.nrows() {
            for col in 0..a.ncols() {
                print!("{}", a[[row, col]]);
            }
            println!();
        }
    }
    fn solved(&self, map: &Array2<Tile>) -> bool {
        for actor in &self.actors {
            match map[actor.idx()] {
                H => return false,
                X => return false,
                W => panic!(),
                R(c) => {
                    if actor.name != c {
                        return false;
                    }
                }
            };
        }
        return true;
    }
    fn stuck(&self) -> bool {
        let stuck_indices = self
            .actors
            .iter()
            .enumerate()
            .filter_map(|(i, a)| {
                if a.has_stopped_in_hall && a.position[1] == 1 {
                    Some(i)
                } else {
                    None
                }
            })
            .collect_vec();
        for &i in &stuck_indices {
            for &j in &stuck_indices {
                if i != j {
                    if self.actors[j].desired_column() <= self.actors[i].position[0]
                        && self.actors[i].desired_column() >= self.actors[j].position[0]
                    {
                        return true;
                    }
                }
            }
        }
        return false;
    }
    fn children(&self, map: &Array2<Tile>) -> Vec<(State, i64)> {
        let mut result = Vec::new();
        let mut valid_actors = 0..self.actors.len();
        if let Some(last) = self.last_moved {
            let actor = &self.actors[last];
            if let X = map[actor.idx()] {
                valid_actors = last..last + 1;
            } else if actor.has_stopped_in_hall {
                valid_actors = last..last + 1;
            }
        }
        if self.stuck() {
            return vec![];
        }
        for actor_idx in valid_actors {
            let actor = &self.actors[actor_idx];
            for offset in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
                let new_idx = actor.position.zip(offset).map(|(a, b)| a + b);
                if let None = map.get(new_idx.map(|x| x as usize)) {
                    continue;
                }
                if self.actors.iter().any(|a| a.position == new_idx) {
                    continue;
                }
                match map[new_idx.map(|x| x as usize)] {
                    W => continue,
                    H | X => {}
                    R(c) => {
                        if c != actor.name && map[actor.idx()] != R(c) {
                            continue;
                        }
                        if self
                            .actors
                            .iter()
                            .any(|a| a != actor && map[a.idx()] == R(c) && a.name != c)
                        {
                            continue;
                        }
                        if actor.position[1] == 3 {
                            continue;
                        }
                    }
                }
                if actor.has_stopped_in_hall
                    && num::signum(actor.desired_column() - actor.position[0]) == -offset[0]
                {
                    continue;
                }
                // Actually compute the effect of moving to new_idx
                let mut new_actors = self.actors.iter().cloned().collect_vec();
                new_actors[actor_idx].position = new_idx;
                if let Some(last) = self.last_moved {
                    if last != actor_idx {
                        if let H | X = map[self.actors[last].idx()] {
                            new_actors[last].has_stopped_in_hall = true;
                        }
                    }
                }
                new_actors.sort();
                result.push((
                    State {
                        actors: new_actors,
                        last_moved: Some(actor_idx),
                    },
                    actor.move_cost(),
                ));
            }
        }
        return result;
    }
}
fn initial_state() -> State {
    State {
        actors: vec![
            Actor::new('A', [2, 3]),
            Actor::new('D', [3, 3]),
            Actor::new('C', [2, 5]),
            Actor::new('D', [3, 5]),
            Actor::new('B', [2, 7]),
            Actor::new('A', [3, 7]),
            Actor::new('B', [2, 9]),
            Actor::new('C', [3, 9]),
        ],
        last_moved: None,
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Node(i64, State);

#[test]
fn test_solved() {
    let map = load_map();
    let state = State {
        actors: vec![
            Actor::new('A', [2, 3]),
            Actor::new('A', [3, 3]),
            Actor::new('B', [2, 5]),
            Actor::new('B', [3, 5]),
            Actor::new('C', [2, 7]),
            Actor::new('C', [3, 7]),
            Actor::new('D', [2, 9]),
            Actor::new('D', [3, 9]),
        ],
        last_moved: None,
    };
    println!("solved? {}", state.solved(&map));
}
#[test]
fn part1() {
    let map = load_map();
    let state = initial_state();
    let mut visited = HashMap::<State, i64>::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(Node(0, state)));
    let mut count = 0;
    while let Some(Reverse(Node(cost, state))) = queue.pop() {
        count += 1;
        if count % 1000 == 0 {
            println!("counted {} {}", count, cost);
        }
        if state.solved(&map) {
            println!("Solved! {}", cost);
            state.display(&map);
            break;
        }
        if visited.get(&state).map(|&v| v < cost).unwrap_or(false) {
            continue;
        }

        for (child, step_cost) in state.children(&map) {
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
}
