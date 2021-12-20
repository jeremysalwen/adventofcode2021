use std::fs::File;
use std::io::{self, BufRead};

fn load_crabs() -> Vec<i64> {
    let file = File::open("7.txt").unwrap();
    let mut lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    return lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();
}
#[test]
fn part1() {
    let mut crabs = load_crabs();
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    let fuel: i64 = crabs.iter().map(|x| num::abs(x - median)).sum();
    println!("Median {} Total fuel used {}", median, fuel);
}

fn pos2score(x: i64, crabs: &Vec<i64>) -> i64 {
    crabs
        .iter()
        .map(|c|
            num::abs(x - c) * (num::abs(x - c) + 1) / 2
        )
        .sum()
}

#[test]
fn part2() {
    let crabs = load_crabs();
    let best_position = (0..2000)
        .map(|x| (x, pos2score(x, &crabs)))
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();
    println!("Best position is {} with fuel {}", best_position.0, best_position.1);
}
