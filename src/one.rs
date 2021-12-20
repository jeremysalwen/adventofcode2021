use std::fs::File;
use std::io::{self, BufRead};

#[test]
fn main() {
    let file = File::open("1.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();

    let readings: Vec<u64> = lines
        .into_iter()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .collect();
    let num_increases = readings
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count();
    let smoothed_readings: Vec<u64> = readings
        .windows(3)
        .map(|window| window.into_iter().sum())
        .collect();
    let num_smoothed_increases = smoothed_readings
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count();
    println!(
        "Number of increases {}, Number of smoothed increases {}",
        num_increases, num_smoothed_increases
    );
}
