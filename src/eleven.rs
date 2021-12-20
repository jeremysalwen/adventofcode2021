use array2d::Array2D;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn load_array() -> Array2D<i64> {
    let file = File::open("11.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();

    return array2d::Array2D::from_rows(
        &lines
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap().into())
                    .collect_vec()
            })
            .collect_vec(),
    );
}

fn apply_step(array: &mut Array2D<i64>) -> i64 {
    let mut popped = Array2D::filled_with(false, array.num_rows(), array.num_columns());
    let mut stack: Vec<(usize, usize)> = vec![];
    for i in 0..array.num_rows() {
        for j in 0..array.num_columns() {
            let val = array.get_mut(i, j).unwrap();
            *val += 1;
            if *val > 9 {
                stack.push((i, j));
                popped.set(i, j, true).unwrap();
            }
        }
    }
    let mut num_pops = 0;
    loop {
        match stack.pop() {
            Some((i, j)) => {
                num_pops += 1;
                for di in -1i64..=1 {
                    for dj in -1i64..=1 {
                        if (di, dj) == (0, 0) {
                            continue;
                        }
                        // Now we POP
                        let (ni, nj) = ((i as i64 + di) as usize, (j as i64 + dj) as usize);
                        match array.get_mut(ni, nj) {
                            Some(neighbor_ref) => {
                                *neighbor_ref += 1;
                                if *neighbor_ref > 9 && !popped[(ni, nj)] {
                                    stack.push((ni, nj));
                                    popped.set(ni, nj, true).unwrap();
                                }
                            }
                            None => {},
                        }
                    }
                }
            }
            None => {
                break;
            }
        }
    }
    for i in 0..array.num_rows() {
        for j in 0..array.num_columns() {
            let val = array.get_mut(i, j).unwrap();
            if *val > 9 {
                *val = 0;
            }
        }
    }
    return num_pops;
}
#[test]
fn part1() {
    let mut array = load_array();
    let mut num_flashes = 0;
    for _ in 0..100 {
        num_flashes += apply_step(&mut array);
    }
    println!("num flashes {}", num_flashes);
}

#[test]
fn part2() {
    let mut array = load_array();
    for i in 0..10000000 {
        let flashes = apply_step(&mut array);
        if flashes == array.num_elements() as i64 {
            println!("Found after step {}", i);
            return;
        }
    }
}
