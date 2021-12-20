use array2d;
use itertools::iproduct;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn load_grid() -> array2d::Array2D<i64> {
    let file = File::open("9.txt").unwrap();
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

#[test]
fn part1() {
    let grid = load_grid();
    let mut danger_level = 0;
    for (row, col) in iproduct!((0..grid.num_rows()), (0..grid.num_columns())) {
        let mut smaller = false;
        let val = grid[(row, col)];
        for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
            match grid.get((row as i32 + dr) as usize, (col as i32 + dc) as usize) {
                Some(&v) => {
                    if v <= val {
                        smaller = true;
                    }
                }
                None => {}
            }
        }
        if !smaller {
            danger_level += 1 + val;
        }
    }
    println!("Grid {:?}", grid);
    println!("Total risk level {}", danger_level);
}

fn basin_size(grid: &array2d::Array2D<i64>, point: (i64, i64)) -> i64 {
    let mut visited = HashSet::<(i64, i64)>::new();
    let mut basin_size = 0;
    let mut queue = vec![point];
    loop {
        match queue.pop() {
            Some(coords) => {
                if !visited.contains(&coords) {
                    visited.insert(coords);
                    match grid.get(coords.0 as usize, coords.1 as usize) {
                        Some(&v) => {
                            if v < 9 {
                                basin_size += 1;
                                for (dr, dc) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)] {
                                    let neighbor_coords = (coords.0 + dr, coords.1 + dc);
                                    if ! visited.contains(&neighbor_coords) {
                                        queue.push(neighbor_coords);
                                    }
                                }
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {
                break;
            }
        }
    }
    return basin_size;
}
#[test]
fn part2() {
    let grid = load_grid();
    let mut basin_sizes = vec![];
    for (row, col) in iproduct!((0..grid.num_rows()), (0..grid.num_columns())) {
        let mut smaller = false;
        let val = grid[(row, col)];
        for (dr, dc) in [(-1i32, 0i32), (1, 0), (0, -1), (0, 1)] {
            match grid.get((row as i32 + dr) as usize, (col as i32 + dc) as usize) {
                Some(&v) => {
                    if v <= val {
                        smaller = true;
                    }
                }
                None => {}
            }
        }
        if !smaller {
            basin_sizes.push(basin_size(&grid, (row as i64, col as i64)));
        }
    }
    basin_sizes.sort_by_key(|x| -x);
    println!("basin sizes {:?}", basin_sizes.iter().take(3).product::<i64>());
}
