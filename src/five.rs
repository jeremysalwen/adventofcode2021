use array2d::Array2D;
use num;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};

struct Line {
    start: (i64, i64),
    end: (i64, i64),
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }
    fn is_diagonal(&self) -> bool {
        num::abs(self.start.1 - self.end.1) == num::abs(self.start.1 - self.end.1)
    }
}

fn load_lines() -> Vec<Line> {
    let file = File::open("5.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    let re = regex::Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    return lines
        .map(|l| {
            let line = l.unwrap();
            let captures = re.captures(line.as_str()).unwrap();
            Line {
                start: (
                    captures.get(1).unwrap().as_str().parse().unwrap(),
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                ),
                end: (
                    captures.get(3).unwrap().as_str().parse().unwrap(),
                    captures.get(4).unwrap().as_str().parse().unwrap(),
                ),
            }
        })
        .collect();
}

fn multi_occupancy_count(lines: Vec<Line>) -> usize {
    let mut occupancy_grid = Array2D::filled_with(0, 1000, 1000);
    for line in lines {
        let mut coords = line.start;
        let delta = (
            num::signum(line.end.0 - line.start.0),
            num::signum(line.end.1 - line.start.1),
        );
        while coords != line.end {
            occupancy_grid[(coords.0.try_into().unwrap(), coords.1.try_into().unwrap())] += 1;
            coords.0 += delta.0;
            coords.1 += delta.1;
        }
        // Include final point
        occupancy_grid[(coords.0.try_into().unwrap(), coords.1.try_into().unwrap())] += 1;
    }
    return occupancy_grid
        .elements_row_major_iter()
        .filter(|&&n| n > 1)
        .count();
}

#[test]
fn part1() {
    let mut lines = load_lines();
    lines.retain(|l| l.is_horizontal() || l.is_vertical());
    let multi_occupied_cells = multi_occupancy_count(lines);
    println!("Number of multi-occupied cells {}", multi_occupied_cells);
}

#[test]
fn part2() {
    let mut lines = load_lines();
    lines.retain(|l| l.is_horizontal() || l.is_vertical() || l.is_diagonal());
    let multi_occupied_cells = multi_occupancy_count(lines);
    println!("Number of multi-occupied cells {}", multi_occupied_cells);
}
