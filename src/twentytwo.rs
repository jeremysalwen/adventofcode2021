use itertools::EitherOrBoth;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Op {
    on: bool,
    lower: [i64; 3],
    upper: [i64; 3],
}

fn load_ops() -> Vec<Op> {
    let file = File::open("22.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();

    let re = Regex::new(r"(on|off) x=(.*)\.\.(.*),y=(.*)\.\.(.*),z=(.*)\.\.(.*)").unwrap();
    return lines
        .map(|l| {
            let line = l.unwrap();
            let captures = re.captures(line.as_str()).unwrap();
            Op {
                on: captures.get(1).unwrap().as_str() == "on",
                lower: [
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                    captures.get(4).unwrap().as_str().parse().unwrap(),
                    captures.get(6).unwrap().as_str().parse().unwrap(),
                ],
                upper: [
                    captures.get(3).unwrap().as_str().parse().unwrap(),
                    captures.get(5).unwrap().as_str().parse().unwrap(),
                    captures.get(7).unwrap().as_str().parse().unwrap(),
                ],
            }
        })
        .collect_vec();
}

#[test]
fn part1() {
    let mut grid = ndarray::Array::from_elem((101, 101, 101), false);
    let ops = load_ops();
    let bounded_ops = ops.iter().map(|op| Op {
        on: op.on,
        lower: op.lower.map(|l| std::cmp::max(l, -50)),
        upper: op.upper.map(|u| std::cmp::min(u, 50)),
    });
    for op in bounded_ops {
        for x in op.lower[0]..=op.upper[0] {
            for y in op.lower[1]..=op.upper[1] {
                for z in op.lower[2]..=op.upper[2] {
                    grid[[(x + 50) as usize, (y + 50) as usize, (z + 50) as usize]] = op.on;
                }
            }
        }
    }
    println!("grid on {}", grid.iter().filter(|b| **b).count());
}

fn cell_width(coords: &EitherOrBoth<&i64, &i64>) -> i64 {
    match coords {
        EitherOrBoth::Both(x, next) => *next - *x,
        EitherOrBoth::Left(_) => 1,
        EitherOrBoth::Right(_) => {
            panic!()
        }
    }
}
#[test]
fn part2() {
    let ops = load_ops();
    let mut coords = [vec![], vec![], vec![]];
    for i in 0..3 {
        coords[i] = ops
            .iter()
            .flat_map(|op| [op.lower[i], op.upper[i] + 1].into_iter())
            .collect();
        coords[i].sort();
        coords[i].dedup();
        println!("coords size {}", coords[i].len());
    }
    let mut grid = ndarray::Array::from_elem::<(usize, usize, usize)>(
        coords.iter().map(|c| c.len()).collect_tuple().unwrap(),
        false,
    );
    let range = |op: &Op, i: usize| {
        coords[i].binary_search(&op.lower[i]).unwrap()
            ..coords[i].binary_search(&(op.upper[i] + 1)).unwrap()
    };
    for (i, op) in ops.iter().enumerate() {
        println!("computing op {}", i);
        for x in range(&op, 0) {
            for y in range(&op, 1) {
                for z in range(&op, 2) {
                    grid[[x, y, z]] = op.on;
                }
            }
        }
    }

    let mut total_on = 0;
    for (x, xsize) in coords[0]
        .iter()
        .zip_longest(coords[0].iter().skip(1))
        .enumerate()
    {
        println!("computing x {} {:?}", x, xsize);
        for (y, ysize) in coords[1]
            .iter()
            .zip_longest(coords[1].iter().skip(1))
            .enumerate()
        {
            for (z, zsize) in coords[2]
                .iter()
                .zip_longest(coords[2].iter().skip(1))
                .enumerate()
            {
                if grid[[x, y, z]] {
                    total_on += cell_width(&xsize) * cell_width(&ysize) * cell_width(&zsize);
                }
            }
        }
    }
    println!("total on {}", total_on);
}
