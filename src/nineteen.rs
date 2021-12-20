use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Scanner {
    points: Vec<[i64; 3]>,
}

trait Transform {
    fn apply(&self, point: &[i64; 3]) -> [i64; 3];
    fn inverse(&self) -> Self;
}

#[derive(Debug, Clone, Copy)]
struct Reflection {
    inversion: [bool; 3],
}

// Technically a permutation of axes, not necessarily a rotation

#[derive(Debug, Clone, Copy)]
struct Rotation {
    permutation: [i8; 3],
}

#[derive(Debug, Clone, Copy)]
struct Translation {
    offset: [i64; 3],
}

#[derive(Debug, Clone, Copy)]
struct Combination {
    rev: bool,
    reflection: Reflection,
    rotation: Rotation,
    translation: Translation,
}

impl Transform for Reflection {
    fn apply(&self, point: &[i64; 3]) -> [i64; 3] {
        [
            if self.inversion[0] {
                -point[0]
            } else {
                point[0]
            },
            if self.inversion[1] {
                -point[1]
            } else {
                point[1]
            },
            if self.inversion[2] {
                -point[2]
            } else {
                point[2]
            },
        ]
    }
    fn inverse(&self) -> Self {
        *self
    }
}

impl Transform for Rotation {
    fn apply(&self, point: &[i64; 3]) -> [i64; 3] {
        let mut result = [0; 3];
        for i in 0..3 {
            result[self.permutation[i] as usize] = point[i];
        }
        result
    }
    fn inverse(&self) -> Self {
        let mut inverse = [0i8; 3];
        for i in 0..3i8 {
            inverse[self.permutation[i as usize] as usize] = i;
        }
        Rotation {
            permutation: inverse,
        }
    }
}

impl Transform for Translation {
    fn apply(&self, point: &[i64; 3]) -> [i64; 3] {
        point.zip(self.offset).map(|(a, b)| a + b)
    }
    fn inverse(&self) -> Self {
        Translation {
            offset: self.offset.map(|x| -x),
        }
    }
}

impl Transform for Combination {
    fn apply(&self, point: &[i64; 3]) -> [i64; 3] {
        if self.rev {
            self.reflection
                .apply(&self.rotation.apply(&self.translation.apply(point)))
        } else {
            self.translation
                .apply(&self.rotation.apply(&self.reflection.apply(point)))
        }
    }
    fn inverse(&self) -> Self {
        Combination {
            rev: !self.rev,
            reflection: self.reflection.inverse(),
            rotation: self.rotation.inverse(),
            translation: self.translation.inverse(),
        }
    }
}

impl Combination {
    fn new(reflection: [bool; 3], rotation: [i8; 3], translation: [i64; 3]) -> Combination {
        Combination {
            rev: false,
            reflection: Reflection {
                inversion: reflection,
            },
            rotation: Rotation {
                permutation: rotation,
            },
            translation: Translation {
                offset: translation,
            },
        }
    }
}
impl Translation {
    fn id() -> Translation {
        Translation { offset: [0, 0, 0] }
    }
}

fn permutation_parity(permutation: &[i8; 3]) -> bool {
    permutation
        .iter()
        .enumerate()
        .filter(|(i, v)| *i as i8 == **v)
        .count()
        == 1
}

fn all_symmetries() -> impl Iterator<Item = (Reflection, Rotation)> {
    let false_true = [false, true];
    let all_permutations = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    return iproduct!(false_true, false_true, all_permutations.into_iter()).map(
        |(invx, invy, permutation)| {
            let parity = invx ^ invy ^ permutation_parity(&permutation);
            (
                Reflection {
                    inversion: [invx, invy, parity],
                },
                Rotation { permutation },
            )
        },
    );
}

fn in_bounds(point: &[i64; 3], min_bounds: &[i64; 3], max_bounds: &[i64; 3]) -> bool {
    point
        .zip(*min_bounds)
        .zip(*max_bounds)
        .iter()
        .all(|((p, lower), upper)| p >= lower && p <= upper)
}

impl Scanner {
    //  Returns the translation to apply to other to match with self.
    fn matches(&self, other: &Scanner, size: i64, min_overlap: i64) -> Option<Combination> {
        let point_set: HashSet<_> = self.points.iter().collect();
        for (reflection, rotation) in all_symmetries() {
            let rotation = Combination {
                rev: false,
                reflection,
                rotation,
                translation: Translation::id(),
            };
            // println!("Rotation {:?}", rotation);
            for point in &self.points {
                for other_point in &other.points {
                    let offset = point.zip(rotation.apply(&other_point)).map(|(p, o)| p - o);
                    // println!("offset {:?}", offset);
                    // println!("points {:?} {:?}", point, other_point);
                    let min_bounds = offset.map(|c| std::cmp::max(-size, c - size));
                    let max_bounds = offset.map(|c| std::cmp::min(size, c + size));
                    // println!("bounds {:?}, {:?}", min_bounds, max_bounds);
                    let transform = Combination {
                        translation: Translation { offset },
                        ..rotation
                    };
                    // check all points in the intersection
                    let mut transformed_other_points = HashSet::new();

                    let mut matches = 0;
                    let mut conflict = false;
                    for p in &other.points {
                        let transformed_point = transform.apply(p);
                        transformed_other_points.insert(transformed_point);
                        // println!("examin point {:?} transformed {:?}", p, transformed_point);
                        if in_bounds(&transformed_point, &min_bounds, &max_bounds) {
                            // println!(
                            //     "in bounds! contained {}",
                            //     point_set.contains(&transformed_point)
                            // );
                            if point_set.contains(&transformed_point) {
                                matches += 1;
                            } else {
                                conflict = true;
                                break;
                            }
                        }
                    }
                    // println!("Transformed other points {:?}", transformed_other_points);
                    if !conflict && matches >= min_overlap {
                        for p in &self.points {
                            // println!("v2 point {:?}", p);
                            if in_bounds(p, &min_bounds, &max_bounds) {
                                if !transformed_other_points.contains(p) {
                                    // println!("conflict!");
                                    conflict = true;
                                    break;
                                }
                            }
                        }
                        if !conflict {
                            return Some(transform);
                        }
                    }
                }
            }
        }
        return None;
    }
}

fn load_scanners() -> Vec<Scanner> {
    let file = File::open("19.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();

    let re = regex::Regex::new("(.*),(.*),(.*)").unwrap();
    let mut scanners = vec![];
    for l in lines {
        let line = l.unwrap();
        if line.starts_with("---") {
            scanners.push(Scanner { points: vec![] });
        } else if !line.is_empty() {
            let captures = re.captures(line.as_str()).unwrap();
            let point: [i64; 3] = captures
                .iter()
                .skip(1)
                .map(|m| m.unwrap().as_str().parse().unwrap())
                .collect_vec()
                .try_into()
                .unwrap();
            scanners.last_mut().unwrap().points.push(point);
        }
    }
    return scanners;
}

#[test]
fn test_transformation() {
    let scanners = load_scanners();
    let transform = Combination::new([false, false, false], [0, 1, 2], [5, 2, 0]);
    assert!(scanners.iter().all(|s| s
        .points
        .iter()
        .all(|p| *p == transform.inverse().apply(&transform.apply(p)))));
    assert!(scanners.iter().all(|s| s
        .points
        .iter()
        .all(|p| *p == transform.apply(&transform.inverse().apply(p)))));
    println!(
        "transformed {:?}",
        scanners[0]
            .points
            .iter()
            .map(|p| transform.apply(p))
            .collect_vec()
    );
}

#[test]
fn test_all_symmetries() {
    let symmetries = all_symmetries().collect_vec();
    println!("symmetries {:?}", symmetries);
    println!("num symmetries {}", symmetries.len());
}

#[test]
fn test_matches_2d() {
    let scanner1 = Scanner {
        points: vec![[0, 2, 0], [4, 1, 0], [3, 3, 0]],
    };
    let scanner2 = Scanner {
        points: vec![[-1, -1, 0], [-5, 0, 0], [-2, 1, 0]],
    };
    println!("Matches {:?}", scanner1.matches(&scanner2, 1000, 3));
}

fn reconstruct(
    scanners: &Vec<Scanner>,
    edges: &HashMap<usize, Vec<(usize, Combination)>>,
) -> (Vec<[i64; 3]>, HashSet<[i64; 3]>) {
    let mut centers = Vec::new();
    let mut points = HashSet::new();
    let mut visited = HashSet::new();
    let mut frontier: Vec<(usize, Vec<&Combination>)> = vec![(0usize, vec![])];
    while let Some((i, transform)) = frontier.pop() {
        println!("visiting {} ", i);
        let mut offset = [0, 0, 0];
        for &t in transform.iter().rev() {
            offset = t.apply(&offset);
            // println!("offset {:?}", offset);
        }
        centers.push(offset);
        for point in &scanners[i].points {
            let mut p = *point;
            for &t in transform.iter().rev() {
                p = t.apply(&p);
            }
            points.insert(p);
        }
        // println!("points {:?}", points);
        visited.insert(i);
        if let Some(children) = edges.get(&i) {
            for (child, child_transform) in children {
                if !visited.contains(child) {
                    let mut new_transform = transform.clone();
                    new_transform.push(child_transform);
                    frontier.push((*child, new_transform));
                }
            }
        }
    }

    return (centers, points);
}
#[test]
fn part12() {
    let scanners = load_scanners();
    let mut edges = HashMap::new();
    for (i, scanner1) in scanners.iter().enumerate() {
        for (j, scanner2) in scanners.iter().enumerate() {
            if i < j {
                println!("Comparing {} and {}", i, j);
                if let Some(transform) = scanner1.matches(&scanner2, 1000, 12) {
                    println!("found match {:?}", transform);

                    edges
                        .entry(i)
                        .or_insert_with(|| vec![])
                        .push((j, transform));
                    edges
                        .entry(j)
                        .or_insert_with(|| vec![])
                        .push((i, transform.inverse()));
                }
            }
        }
    }
    println!("matches {:?}", edges);
    let (centers, reconstructed) = reconstruct(&scanners, &edges);
    println!("reconstructed {:?}", reconstructed);
    println!("size {}", reconstructed.len());

    let max_distance: i64 = centers
        .iter()
        .filter_map(|c1| {
            centers
                .iter()
                .map(|c2| c1.zip(*c2).map(|(a, b)| num::abs(a - b)).iter().sum())
                .max()
        })
        .max()
        .unwrap();
    println!("max distance {}", max_distance);
}
