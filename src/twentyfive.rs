use itertools::Itertools;
use ndarray::Array2;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Right,
    Down,
}

impl Tile {
    fn parse(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '>' => Tile::Right,
            'v' => Tile::Down,
            _ => panic!(),
        }
    }
}

fn load_grid() -> Array2<Tile> {
    let file = File::open("25.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    let arr = lines
        .map(|l| l.unwrap().chars().map(Tile::parse).collect_vec())
        .collect_vec();
    return ndarray::Array2::from_shape_fn((arr.len(), arr[0].len()), |(i, j)| arr[i][j]);
}

struct Board {
    grid: Array2<Tile>,
    active_right: HashSet<[usize; 2]>,
    active_down: HashSet<[usize; 2]>,
}

impl Board {
    fn from_grid(grid: &Array2<Tile>) -> Board {
        Board {
            grid: grid.clone(),
            active_right: grid
                .indexed_iter()
                .filter_map(|((row, col), t)| match t {
                    Tile::Right => Some([row, col]),
                    _ => None,
                })
                .collect(),
            active_down: grid
                .indexed_iter()
                .filter_map(|((row, col), t)| match t {
                    Tile::Down => Some([row, col]),
                    _ => None,
                })
                .collect(),
        }
    }
    fn get_queue(&self, direction: Tile) -> &HashSet<[usize; 2]> {
        match direction {
            Tile::Right => &self.active_right,
            Tile::Down => &self.active_down,
            Tile::Empty => panic!(),
        }
    }

    fn get_queue_mut(&mut self, direction: Tile) -> &mut HashSet<[usize; 2]> {
        match direction {
            Tile::Right => &mut self.active_right,
            Tile::Down => &mut self.active_down,
            Tile::Empty => panic!(),
        }
    }
    fn next(&self, ind: &[usize; 2]) -> [usize; 2] {
        match self.grid[*ind] {
            Tile::Right => self.right(ind),
            Tile::Down => self.down(ind),
            _ => panic!(),
        }
    }
    fn right(&self, ind: &[usize; 2]) -> [usize; 2] {
        [ind[0], (ind[1] + 1) % self.grid.ncols()]
    }

    fn down(&self, ind: &[usize; 2]) -> [usize; 2] {
        [(ind[0] + 1) % self.grid.nrows(), ind[1]]
    }
    fn left(&self, ind: &[usize; 2]) -> [usize; 2] {
        [ind[0], (ind[1] + self.grid.ncols() - 1) % self.grid.ncols()]
    }
    fn up(&self, ind: &[usize; 2]) -> [usize; 2] {
        [(ind[0] + self.grid.nrows() - 1) % self.grid.nrows(), ind[1]]
    }
}

fn step(board: &mut Board) -> bool {
    let mut changed = false;
    for direction in [Tile::Right, Tile::Down] {
        let stepped = board
            .get_queue(direction)
            .iter()
            .cloned()
            .filter(|&ind| board.grid[board.next(&ind)] == Tile::Empty)
            .collect_vec();
        board.get_queue_mut(direction).clear();
        for start in stepped {
            changed = true;
            let direction = board.grid[start];
            let end = board.next(&start);
            board.grid[start] = Tile::Empty;
            board.grid[end] = direction;
            board.get_queue_mut(direction).insert(end);
            let above = board.up(&start);
            if board.grid[above] == Tile::Down {
                board.active_down.insert(above);
            }
            let left = board.left(&start);
            if board.grid[left] == Tile::Right {
                board.active_right.insert(left);
            }
        }
    }
    return changed;
}
#[test]
fn part1() {
    let grid = load_grid();
    let mut board = Board::from_grid(&grid);
    println!("{:?}", grid);
    let mut steps = 1;
    while step(&mut board) {
        steps += 1;
        if steps % 100 == 0 {
            println!(
                "Step {} {} {}",
                steps,
                board.active_right.len(),
                board.active_down.len()
            );
        }
    }
    println!("steps till stop {}", steps);
}
