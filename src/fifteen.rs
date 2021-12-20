use array2d::Array2D;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};

fn load_array() -> Array2D<i8> {
    let file = File::open("15.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();

    return array2d::Array2D::from_rows(
        &lines
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i8)
                    .collect_vec()
            })
            .collect_vec(),
    );
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    value: i64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .value
            .cmp(&self.value)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn djikstras(array: &array2d::Array2D<i8>, start: (usize, usize), end: (usize, usize)) -> i64 {
    let mut queue = std::collections::BinaryHeap::new();
    queue.push(Node {
        position: start,
        value: 0,
    });
    let mut visited = array2d::Array2D::filled_with(false, array.num_rows(), array.num_columns());
    while let Some(Node {
        position: (row, col),
        value,
    }) = queue.pop()
    {
        if (row, col) == end {
            return value;
        }
        if *visited.get(row, col).unwrap() {
            continue;
        }
        visited.set(row, col, true).unwrap();
        for (di, dj) in [(-1i64, 0i64), (1, 0), (0, 1), (0, -1)] {
            let (ni, nj) = ((row as i64 + di) as usize, (col as i64 + dj) as usize);

            if let Some(val) = array.get(ni, nj) {
                queue.push(Node {
                    position: (ni, nj),
                    value: value + *val as i64,
                })
            }
        }
    }
    panic!();
}

#[test]
fn part1() {
    let array = load_array();
    let dist = djikstras(
        &array,
        (0, 0),
        (array.num_rows() - 1, array.num_columns() - 1),
    );
    println!("dist {:?}", dist);
}

fn expand_array(array: &Array2D<i8>) -> Array2D<i8> {
    let mult = 5;
    let mut new_array =
        Array2D::filled_with(0, array.num_rows() * mult, array.num_columns() * mult);
    for ix in 0..mult {
        for jx in 0..mult {
            for row in 0..array.num_rows() {
                for col in 0..array.num_columns() {
                    let mut new_val = array[(row, col)] + ix as i8 + jx as i8;
                    if new_val > 9 {
                        new_val -= 9;
                    }
                    new_array[(ix * array.num_rows() + row, jx * array.num_columns() + col)] =
                        new_val;
                }
            }
        }
    }
    return new_array;
}

#[test]
fn part2() {
    let array = load_array();
    let duped = expand_array(&array);
    let dist = djikstras(
        &duped,
        (0, 0),
        (duped.num_rows() - 1, duped.num_columns() - 1),
    );
    println!("dist {:?}", dist);
}
