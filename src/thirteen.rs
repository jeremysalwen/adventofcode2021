use array2d::Array2D;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone,  PartialEq, Eq, PartialOrd, Ord)]
enum Fold {
    X(i64),
    Y(i64),
}
impl Fold {
    fn parse(line: &str) -> Fold {
        let re = regex::Regex::new("fold along (.)=(.*)").unwrap();
        let matches = re.captures(line).unwrap();
        let offset: i64 = matches.get(2).unwrap().as_str().parse().unwrap();
        if matches.get(1).unwrap().as_str() == "x" {
            Fold::X(offset)
        } else {
            Fold::Y(offset)
        }
    }
    fn apply(&self, point: (i64, i64)) -> (i64, i64) {
        match self {
            Fold::X(x) => (x - num::abs(point.0 - x), point.1),
            Fold::Y(y) => (point.0, y - num::abs(point.1 - y)),
        }
    }
}
fn load_points() -> (Vec<(i64, i64)>, Vec<Fold>) {
    let file = File::open("13.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();

    let re = regex::Regex::new("(.*),(.*)").unwrap();
    let mut points = vec![];
    let mut folds = vec![];
    let mut passed_mid = false;
    for l in lines {
        let line = l.unwrap();
        if line.is_empty() {
            passed_mid = true;
            continue;
        }

        if !passed_mid {
            let matches = re.captures(line.as_str()).unwrap();
            points.push((
                matches.get(1).unwrap().as_str().parse().unwrap(),
                matches.get(2).unwrap().as_str().parse().unwrap(),
            ));
        } else {
            folds.push(Fold::parse(line.as_str()));
        }
    }
    return (points, folds);
}

#[test]
fn part1() {
    let (mut points, folds) = load_points();
    for fold in folds.iter().take(1) {
        points = points.into_iter().map(|p| fold.apply(p)).collect();
    }
    let mut unique_points = points.iter().unique().collect_vec();
    unique_points.sort();
    println!("unique_points {:?}", unique_points);
    println!("count {}", unique_points.len());
    println!("folds {:?}", folds);
}

#[test]
fn part2() {
    let (mut points, folds) = load_points();
    for fold in folds.iter() {
        points = points.into_iter().map(|p| fold.apply(p)).collect();
    }

    let max_x = points.iter().map(|p| p.0).max().unwrap() + 1;
    let max_y = points.iter().map(|p| p.1).max().unwrap() + 1;
    let mut arr = Array2D::filled_with(false, max_y.try_into().unwrap(), max_x.try_into().unwrap());
    for (x,y) in points {
        arr.set(y.try_into().unwrap(), x.try_into().unwrap(), true).unwrap();
    }    
    for row_iter in arr.rows_iter() {
        for element in row_iter {
            print!("{} ", if *element {"#"} else {" "});
        }
        println!();
    }
}