use array2d::Array2D;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

struct Board {
    numbers: Array2D<u64>,
    filled: Array2D<bool>,
}

impl Board {
    fn new(rows: Vec<Vec<u64>>) -> Board {
        Board {
            numbers: Array2D::from_rows(&rows),
            filled: Array2D::filled_by_row_major(|| false, rows.len(), rows[0].len()),
        }
    }

    fn fill(&mut self, n: u64) -> bool {
        let mut winner = false;
        for row in 0..self.numbers.num_rows() {
            for col in 0..self.numbers.num_columns() {
                if self.numbers[(row, col)] == n {
                    self.filled[(row, col)] = true;
                    if self.check_winner(row, col) {
                        winner = true;
                    }
                }
            }
        }
        return winner;
    }
    fn check_winner(&self, row: usize, col: usize) -> bool {
        if self.filled.row_iter(row).all(|&b| b) {
            return true;
        } else if self.filled.column_iter(col).all(|&b| b) {
            return true;
        } else if row == col {
            let mut increasing_diag = true;
            let mut decreasing_diag = true;
            for i in 0..5 {
                if !self.filled[(i, i)] {
                    decreasing_diag = false;
                }
                if !self.filled[(4 - i, i)] {
                    increasing_diag = false;
                }
            }
            if increasing_diag || decreasing_diag {
                return true;
            }
        }
        return false;
    }
    fn unmarked_sum(&self) -> u64 {
        self.numbers
            .elements_row_major_iter()
            .zip(self.filled.elements_row_major_iter())
            .filter_map(|(&n, &filled)| if filled { None } else { Some(n) })
            .sum()
    }
}

fn load_boards() ->(Vec<u64>, Vec<Board>) {
    let file = File::open("4.txt").unwrap();
    let mut lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    let bingo_nums: Vec<u64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let whitespace = regex::Regex::new(" +").unwrap();

    let boards: Vec<Board> = lines
        .chunks(6)
        .into_iter()
        .map(|chunk| {
            Board::new(
                chunk
                    .into_iter()
                    .skip(1)
                    .map(|r| {
                        whitespace
                            .split(&r.unwrap().trim())
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect()
                    })
                    .collect(),
            )
        })
        .collect();
    return (bingo_nums, boards);
}
#[test]
fn part1() {
    let (bingo_nums,mut boards) = load_boards();
    for num in bingo_nums {
        for board in &mut boards {
            if board.fill(num) {
                let unmarked_sum = board.unmarked_sum();
                println!(
                    "Found winning board. Number: {} Unmarked Sum {} Product {}",
                    num,
                    unmarked_sum,
                    num * unmarked_sum
                );
                return;
            }
        }
    }
}
#[test]
fn part2() {
    let (bingo_nums,mut boards) = load_boards();
    for num in bingo_nums {
        if boards.len() == 1 {
            if boards[0].fill(num) {
                let unmarked_sum = boards[0].unmarked_sum();
                println!(
                    "Found winning board. Number: {} Unmarked Sum {} Product {}",
                    num,
                    unmarked_sum,
                    num * unmarked_sum
                );
                return;
            }
        } else {
            boards = boards.into_iter().filter_map(|mut board| if board.fill(num) {None} else {Some(board)}).collect();
        }       
    }
}
