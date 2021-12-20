use derive_more::{Add, Sum};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, PartialEq, Add, Sum)]
struct Point {
    x: i64,
    y: i64,
}

#[test]
fn part1() {
    let file = File::open("2.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    let re = Regex::new(r"(\w+) (\d+)").unwrap();
    let total: Point = lines
        .map(|l| {
            let line = l.unwrap();
            let captures = re.captures(line.as_str()).unwrap();
            let num = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
            match captures.get(1).unwrap().as_str().as_ref() {
                "up" => Point { x: 0, y: -num },
                "down" => Point { x: 0, y: num },
                "forward" => Point { x: num, y: 0 },
                x => panic!("something else! {}", x),
            }
        })
        .sum();
    println!("Total {:?} product {:?}", total, total.x * total.y);
}

#[test]
fn part2() {
    let file = File::open("2.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    let re = Regex::new(r"(\w+) (\d+)").unwrap();
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;
    for l in lines {
        let line = l.unwrap();
        let captures = re.captures(line.as_str()).unwrap();
        let num = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
        match captures.get(1).unwrap().as_str().as_ref() {
            "up" => aim -= num,
            "down" => aim += num,
            "forward" => {
                x += num;
                y += num * aim;
            }
            x => panic!("something else! {}", x),
        }
    }
    println!("x {:?} y {:?} product {:?}", x, y, x * y);
}
