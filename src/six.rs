use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn load_fish() -> Vec<u64> {
    let file = File::open("6.txt").unwrap();
    let mut lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    let fish:Vec<usize> = lines.next().unwrap().unwrap().split(",").into_iter().map(|s| s.parse().unwrap()).collect_vec();
    let mut by_age = vec![0; 9];
    for f in fish {
        by_age[f] += 1;
    }
    return by_age;
}

fn age(by_age: &mut Vec<u64>, days: u64) {
    for _ in 0..days {
        by_age.rotate_left(1);
        by_age[6] += by_age[8];
    }
}

#[test]
fn part1() {
    let mut by_age = load_fish();
    age(&mut by_age, 80);

    let total_fish:u64 = by_age.iter().sum();
    println!("total fish {:}", total_fish);
}

#[test]
fn part2() {
    let mut by_age = load_fish();
    age(&mut by_age, 256);

    let total_fish:u64 = by_age.iter().sum();
    println!("total fish {:}", total_fish);
}