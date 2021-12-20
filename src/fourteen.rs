use counter::Counter;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn load_data() -> (String, Vec<(String, String)>) {
    let file = File::open("14.txt").unwrap();
    let mut lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    let start = lines.next().unwrap().unwrap();
    lines.next().unwrap().unwrap();

    let re = regex::Regex::new("(.*) -> (.*)").unwrap();
    let mut transforms = vec![];
    for l in lines {
        let line = l.unwrap();

        let matches = re.captures(line.as_str()).unwrap();
        transforms.push((
            matches.get(1).unwrap().as_str().to_string(),
            matches.get(2).unwrap().as_str().to_string(),
        ));
    }
    return (start, transforms);
}

#[test]
fn part1() {
    let (start, transforms) = load_data();
    let transform_lookup: HashMap<String, String> = HashMap::from_iter(transforms.iter().cloned());
    println!("transforms {:?}", transform_lookup);
    let mut string = start.clone();
    for _ in 0..10 {
        let insertions = string
            .chars()
            .zip(string.chars().skip(1))
            .map(|(a, b)| {
                transform_lookup
                    .get(&format!("{}{}", a, b))
                    .unwrap_or(&"".to_string())
                    .clone()
            })
            .collect_vec();
        println!("Insertions {:?}", &insertions);
        string = string
            .chars()
            .map(|c| c.to_string())
            .interleave(insertions)
            .collect();

        println!("AFTER {}", string);
    }
    let char_counts = string.chars().collect::<Counter<_>>();
    let max_count = *char_counts.values().max().unwrap();
    let min_count = *char_counts.values().min().unwrap();
    println!("char counts {:?}", char_counts);
    println!("max minus min {}", max_count - min_count);
}

#[test]
fn part2() {
    let (start, transforms) = load_data();
    let transform_lookup: HashMap<(char, char), char> =
        HashMap::from_iter(transforms.iter().map(|(s, s2)| {
            (
                (s.chars().nth(0).unwrap(), s.chars().nth(1).unwrap()),
                s2.chars().nth(0).unwrap(),
            )
        }));
    let mut molecule_count = HashMap::<(char, char), i128>::new();
    for (a, b) in start.chars().zip(start.chars().skip(1)) {
        *molecule_count.entry((a, b)).or_insert(0) += 1;
    }
    for _ in 0..40 {
        let mut new_molecule_count= HashMap::new();
        for ((a, b), count) in molecule_count {
            match transform_lookup.get(&(a, b)) {
                Some(middle) => {
                    *new_molecule_count.entry((a, *middle)).or_insert(0) += count;
                    *new_molecule_count.entry((*middle, b)).or_insert(0) += count;
                }
                None => {
                    *new_molecule_count.entry((a, b)).or_insert(0) += count;
                }
            }
        }
        molecule_count = new_molecule_count;
    }
    let mut char_counts = HashMap::new();
    for ((a,b),&c) in molecule_count.iter() {
        *char_counts.entry(*a).or_insert(0)+=c;
        *char_counts.entry(*b).or_insert(0)+=c;
    }
    *char_counts.entry(start.chars().nth(0).unwrap()).or_insert(0)+=1;
    *char_counts.entry(start.chars().last().unwrap()).or_insert(0)+=1;
    for entry in &mut char_counts {
        *entry.1 /= 2;
    }
    let max_count = *char_counts.values().max().unwrap();
    let min_count = *char_counts.values().min().unwrap();
    println!("max minus min {}", max_count - min_count);
}
