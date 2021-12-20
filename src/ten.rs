use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn load_lines() -> Vec<String> {
    let file = File::open("10.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();

    return lines.map(|l| l.unwrap()).collect_vec();
}

fn bad_indices(lines: &Vec<String>) -> Vec<Option<i32>> {
    let opening_chars = HashSet::from(['[', '(', '<', '{']);
    let closing_chars = HashMap::from([(']', '['), (')', '('), ('>', '<'), ('}', '{')]);
    let mut result = Vec::new();
    for line in lines {
        let mut stack = Vec::<char>::new();
        let mut bad = false;
        for (i, c) in line.chars().enumerate() {
            if opening_chars.contains(&c) {
                stack.push(c);
            } else {
                let opening = closing_chars.get(&c).unwrap();
                if stack.last() != Some(opening) {
                    result.push(Some(i as i32));
                    bad = true;
                    break;
                }
                stack.pop();
            }
        }
        if !bad {
            result.push(None);
        }
    }
    return result;
}
#[test]
fn part1() {
    let lines = load_lines();
    let bad = bad_indices(&lines);
    let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    println!("Bad {:?}", bad);
    let total_score: i32 = lines
        .iter()
        .zip_eq(bad.iter())
        .map(|(l, i)| match i {
            Some(index) => scores
                .get(&l.chars().nth(*index as usize).unwrap())
                .unwrap(),
            None => &0,
        })
        .sum();
    println!("total score is {}", total_score);
}

fn line_score(line: &String) -> Option<i64> {
    let closing_chars =
        bimap::BiHashMap::<char, char>::from_iter([(']', '['), (')', '('), ('>', '<'), ('}', '{')]);

    let mut stack = Vec::<char>::new();
    for c in line.chars() {
        if closing_chars.contains_right(&c) {
            stack.push(c);
        } else {
            let opening = closing_chars.get_by_left(&c).unwrap();
            if stack.last() != Some(opening) {
                return None;
            }
            stack.pop();
        }
    }
    // Score the remaining stack
    let score_per_char = HashMap::from([('(',1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut score = 0;
    println!("Stack {:?}", stack);
    for entry in stack.iter().rev() {
        println!("score {}", score);
        score *= 5;
        score += score_per_char.get(entry).unwrap();
    }
    return Some(score);
}
#[test]
fn part2() {
    let lines = load_lines();
    let mut scores = lines.iter().filter_map(line_score).collect_vec();
    scores.sort();
    let median_score = scores[scores.len()/2];
    println!("final score is {}", median_score);
}
