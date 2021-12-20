use itertools::Itertools;
use multimap::MultiMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Node {
    Start,
    End,
    Small(String),
    Big(String),
}

impl Node {
    fn parse(name: &str) -> Node {
        if name == "start" {
            Node::Start
        } else if name == "end" {
            Node::End
        } else if name == name.to_uppercase() {
            Node::Big(name.to_string())
        } else {
            Node::Small(name.to_string())
        }
    }
}

fn load_edges() -> Vec<(Node, Node)> {
    let file = File::open("12.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();

    let re = regex::Regex::new("(.*)-(.*)").unwrap();
    return lines
        .map(|l| {
            let line = l.unwrap();
            let matches = re.captures(line.as_str()).unwrap();
            (
                Node::parse(matches.get(1).unwrap().as_str()),
                Node::parse(matches.get(2).unwrap().as_str()),
            )
        })
        .collect_vec();
}

fn neighbors_paths(
    starting: &Node,
    visited: &mut HashSet<Node>,
    map: &MultiMap<Node, Node>,
    visited_twice: bool,
) -> i64 {
    let neighbors = map.get_vec(starting).unwrap();
    neighbors
        .iter()
        .map(|n| num_paths(n, visited, map, visited_twice))
        .sum()
}
fn num_paths(
    starting: &Node,
    visited: &mut HashSet<Node>,
    map: &MultiMap<Node, Node>,
    visited_twice: bool,
) -> i64 {
    //println!("Visiting {:?} {:?} {}", starting, visited, visited_twice);
    let already_visited = visited.contains(starting);
    visited.insert(starting.clone());

    let result = match &starting {
        Node::End => 1,
        Node::Start => {
            if already_visited {
                0
            } else {
                neighbors_paths(starting, visited, map, already_visited)
            }
        }
        Node::Small(_) => {
            if already_visited && visited_twice  {
                0
            } else {
                neighbors_paths(starting, visited, map, visited_twice | already_visited)
            }
        }
        Node::Big(_) => {
            neighbors_paths(starting, visited, map, visited_twice)
        }
    };
    if !already_visited {
        visited.remove(starting);
    }
    //println!("result for {:?}, {}", starting, result);
    return result;
}
#[test]
fn part1() {
    let edges = load_edges();
    let mut map = multimap::MultiMap::new();
    for (k, v) in &edges {
        map.insert(k.clone(), v.clone());
        map.insert(v.clone(), k.clone());
    }
    println!("map {:?}", map);
    let npaths = num_paths(&Node::Start, &mut HashSet::new(), &map, false);
    println!("Num paths {}", npaths);
}
