use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct DeterministicDie {
    value: i128,
}

trait Die {
    fn roll(&mut self) -> i128;
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> i128 {
        let result = self.value;
        self.value += 1;
        return result;
    }
}

fn turn<T: Die>(position: &mut i128, score: &mut i128, die: &mut T) {
    for _ in 0..3 {
        *position += die.roll();
        *position = (*position - 1) % 10 + 1;
    }
    *score += *position;
    println!("score {}", *score);
}
fn play_game<T: Die>(mut start1: i128, mut start2: i128, die: &mut T) -> (i128, i128) {
    let mut player1_score = 0;
    let mut player2_score = 0;
    for i in 0..1000 {
        turn(&mut start1, &mut player1_score, die);
        if player1_score >= 1000 {
            return (player2_score, i * 2 + 1);
        }
        turn(&mut start2, &mut player2_score, die);
        if player2_score >= 1000 {
            return (player1_score, i * 2 + 2);
        }
    }
    panic!();
}

#[test]
fn part1() {
    let mut die = DeterministicDie { value: 1 };
    let (score, turns) = play_game(4, 2, &mut die);
    println!("{} {} {} {}", score, turns, turns * 3, score * turns * 3);
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct GameState {
    position1: i8,
    position2: i8,
    score1: i128,
    score2: i128,
}

fn step_world(worlds: &HashMap<GameState, i128>) -> (HashMap<GameState, i128>, i128, i128) {
    let mut player1_wins = 0;
    let mut player2_wins = 0;
    let roll_distribution = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];
    let mut result = HashMap::new();
    for (state, count) in worlds.iter() {
        for (roll1, prob1) in roll_distribution.iter().enumerate() {
            if *prob1 == 0 {
                continue;
            }
            let position1 = ((state.position1 as i128 + roll1 as i128 - 1) % 10 + 1) as i8;
            let score1 = state.score1 + position1 as i128;
            if score1 >= 21 {
                player1_wins += count * prob1;
            } else {
                for (roll2, prob2) in roll_distribution.iter().enumerate() {
                    if *prob2 == 0 {
                        continue;
                    }
                    let position2 = ((state.position2 as i128 + roll2 as i128 - 1) % 10 + 1) as i8;
                    let score2 = state.score2 + position2 as i128;
                    if score2 >= 21 {
                        player2_wins += count * prob1 * prob2;
                    } else {
                            * result
                                .entry(GameState {
                                    position1,
                                    position2,
                                    score1,
                                    score2,
                                })
                                .or_insert(0) += count * prob1 * prob2;
                    }
                }
            }
        }
    }
    return (result, player1_wins, player2_wins);
}

#[test]
fn part2() {
    let mut worlds = HashMap::new();
    worlds.insert(
        GameState {
            position1: 4,
            position2: 2,
            score1: 0,
            score2: 0,
        },
        1,
    );

    let mut totalp1_wins = 0;
    let mut totalp2_wins = 0;
    for i in 0..21 {
        let (new_worlds, p1wins, p2wins) = step_world(&worlds);
        worlds = new_worlds;
        totalp1_wins += p1wins;
        totalp2_wins += p2wins;
    }
    println!("totalwins {} {}", totalp1_wins, totalp2_wins);
}
