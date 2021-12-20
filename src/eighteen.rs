use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum Node {
    Pair(Box<Node>, Box<Node>),
    Literal(i64),
}

impl Node {
    /// Returns the node, and the end index in the string.
    fn parse(string: &[u8], start: usize) -> (Node, usize) {
        if string[start] as char == '[' {
            let (left, ind) = Node::parse(string, start + 1);
            assert_eq!(string[ind] as char, ',');
            let (right, last_ind) = Node::parse(string, ind + 1);
            assert_eq!(string[last_ind] as char, ']');
            return (Node::Pair(Box::from(left), Box::from(right)), last_ind + 1);
        } else {
            let end_ind = start
                + string
                    .iter()
                    .skip(start)
                    .position(|&c| c == b',' || c == b']')
                    .unwrap_or(string.len());
            return (
                Node::Literal(
                    std::str::from_utf8(&string[start..end_ind])
                        .unwrap()
                        .parse()
                        .unwrap(),
                ),
                end_ind,
            );
        }
    }
    fn add_left(&mut self, val: i64) {
        match self {
            Node::Pair(left, _) => {
                left.add_left(val);
            }
            Node::Literal(node_val) => {
                *node_val += val;
            }
        }
    }
    fn add_right(&mut self, val: i64) {
        match self {
            Node::Pair(_, right) => {
                right.add_right(val);
            }
            Node::Literal(node_val) => {
                *node_val += val;
            }
        }
    }

    fn reduce_explode(&mut self, depth: usize) -> Option<(Option<i64>, Option<i64>)> {
        match self {
            Node::Pair(left, right) => {
                if depth >= 4 {
                    if let Node::Literal(left_val) = **left {
                        if let Node::Literal(right_val) = **right {
                            let (l, r) = (left_val, right_val);
                            *self = Node::Literal(0);
                            return Some((Some(l), Some(r)));
                        }
                    }
                }
                if let Some(left_explode) = left.reduce_explode(depth + 1) {
                    if let Some(right_increment) = left_explode.1 {
                        right.add_left(right_increment);
                    }
                    return Some((left_explode.0, None));
                }
                if let Some(right_explode) = right.reduce_explode(depth + 1) {
                    if let Some(left_increment) = right_explode.0 {
                        left.add_right(left_increment);
                    }
                    return Some((None, right_explode.1));
                }
                return None;
            }
            Node::Literal(_) => {
                return None;
            }
        }
    }

    fn split_first(&mut self) -> bool {
        match self {
            Node::Literal(val) => {
                if *val >= 10 {
                    *self = Node::Pair(
                        Box::from(Node::Literal(*val / 2)),
                        Box::from(Node::Literal(*val - *val / 2)),
                    );
                    return true;
                }
            }
            Node::Pair(left, right) => {
                if left.split_first() {
                    return true;
                } else if right.split_first() {
                    return true;
                }
            }
        }
        return false;
    }
    fn reduce(&mut self) {
        loop {
            if let Some(_) = self.reduce_explode(0) {
                continue;
            }
            if self.split_first() {
                continue;
            }
            break;
        }
    }

    fn add(self, other: Node) -> Node {
        let mut result = Node::Pair(Box::from(self), Box::from(other));
        result.reduce();
        result
    }

    fn magnitude(&self) -> i64 {
        match self {
            Node::Literal(val) => *val,
            Node::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn expression(&self) -> String {
        match self {
            Node::Literal(val) => val.to_string(),
            Node::Pair(left, right) => format!("[{},{}]", left.expression(), right.expression()),
        }
    }
}

fn load_expressions() -> Vec<Node> {
    let file = File::open("18.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();

    let expressions = lines
        .map(|l| Node::parse(l.unwrap().as_bytes(), 0).0)
        .collect_vec();
    return expressions;
}

#[test]
fn part1() {
    let expressions = load_expressions();
    let sum = expressions.into_iter().reduce(|sum, n| sum.add(n)).unwrap();
    println!("sum {}", sum.expression());
    println!("magnitude {}", sum.magnitude());
}

#[test]
fn part2() {
    let expressions = load_expressions();
    let mut max_sum = 0;
    for i in 0..expressions.len() {
        for j in 0..expressions.len() {
            if i != j {
                let sum = expressions[i]
                    .clone()
                    .add(expressions[j].clone())
                    .magnitude();
                max_sum = std::cmp::max(sum, max_sum);
            }
        }
    }
    println!("max sum {}", max_sum);
}
