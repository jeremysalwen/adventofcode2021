use array2d;
use array2d::Array2D;
use itertools::iproduct;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn load_grid() -> (Vec<bool>, array2d::Array2D<bool>) {
    let file = File::open("20.txt").unwrap();
    let mut lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    let mut key = Vec::<bool>::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        key.extend(line.chars().map(|c| c == '#'));
    }

    return (
        key,
        array2d::Array2D::from_rows(
            &lines
                .map(|l| l.unwrap().chars().map(|c| c == '#').collect_vec())
                .collect_vec(),
        ),
    );
}

fn neighborhood(grid: &Array2D<bool>, ind: (i64, i64), background: bool) -> u64 {
    let mut result: u64 = 0;
    for drow in -1i64..=1 {
        for dcol in -1i64..=1 {
            result *= 2;
            let (r, c) = ((ind.0 + drow) as usize, (ind.1 + dcol) as usize);
            result += *grid.get(r, c).unwrap_or(&background) as u64;
        }
    }
    return result;
}

fn step(key: &Vec<bool>, grid: &Array2D<bool>, background: bool) -> (Array2D<bool>, bool) {
    let mut result = Array2D::filled_with(false, grid.num_rows() + 2, grid.num_columns() + 2);
    for row in -1..=grid.num_rows() as i64 {
        for col in -1..=grid.num_columns() as i64 {
            result[((row + 1) as usize, (col + 1) as usize)] =
                key[neighborhood(grid, (row, col), background) as usize];
        }
    }
    return (
        result,
        if background {
            key[key.len() - 1]
        } else {
            key[0]
        },
    );
}
#[test]
fn part1() {
    let (key, grid) = load_grid();
    println!("key.size {}", key.len());
    let step1 = step(&key, &grid, false);
    let step2 = step(&key, &step1.0, step1.1);
    println!(
        "lit {}",
        step2.0.elements_row_major_iter().filter(|b| **b).count()
    );
}

#[test]
fn part2() {
    let (key, grid) = load_grid();
    println!("key.size {}", key.len());
    let mut it = (grid, false);
    for _ in 0..50 {
        it = step(&key, &it.0, it.1);
    }
    println!(
        "lit {}",
        it.0.elements_row_major_iter().filter(|b| **b).count()
    );
}
