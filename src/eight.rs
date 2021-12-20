use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use z3::ast;
use z3::ast::Ast;
use z3d::{dec, exp};

fn load_digits() -> Vec<(Vec<String>, Vec<String>)> {
    let file = File::open("8.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();
    return lines
        .map(|l| {
            l.unwrap()
                .split("|")
                .map(|s| s.trim().split(" ").map(|s| s.to_string()).collect())
                .collect_tuple()
                .unwrap()
        })
        .collect();
}

fn decode(examples: &Vec<String>, query: &Vec<String>) -> String {
    let z3_conf = z3::Config::new();
    let ctx = &z3::Context::new(&z3_conf);
    let solver = z3::Solver::new(&ctx);

    let all_words = examples.iter().chain(query.iter()).collect_vec();
    let digits = (0..examples.len() + query.len())
        .map(|i| dec!($("digit_{}", i):bitvec<10> in ctx))
        .collect_vec();
    let mapping: Vec<z3::ast::BV> = (0..7)
        .map(|i| dec!($("mapping_{}", i): bitvec<7> in ctx))
        .collect();

    // Assert mapping and digits are onehot
    let zero = exp!(0 as bitvec<1> in ctx);
    let one = exp!(1 as bitvec<1> in ctx);
    for m in mapping.iter() {
        for offset in 1..7 {
            solver.assert(
                &m.bvrotl(&ast::BV::from_u64(ctx, offset, 7))
                    .bvand(m)
                    .bvredor()
                    ._eq(&zero),
            );
        }
    }
    for d in digits.iter() {
        solver.assert(&d.bvredor()._eq(&one));
        for offset in 1..9 {
            solver.assert(
                &d.bvrotl(&ast::BV::from_u64(ctx, offset, 10))
                    .bvand(d)
                    .bvredor()
                    ._eq(&zero),
            );
        }
    }

    // Assert mapping is unique permutation
    solver.assert(
        &mapping
            .iter()
            .map(|x| x.clone())
            .reduce(|a, e| a.bvor(&e))
            .unwrap()
            .bvredand()
            ._eq(&one),
    );

    // Assert digits include the right segments
    let lit_up_segments = &[
        0b1110111, 0b0010010, 0b1011101, 0b1011011, 0b0111010, 0b1101011, 0b1101111, 0b1010010,
        0b1111111, 0b1111011,
    ];
    for (i, &lit_up_seg) in lit_up_segments.iter().enumerate() {
        let i_as_bv = ast::BV::from_u64(ctx, (1 << (9-i)).try_into().unwrap(), 10);
        let lit_up = ast::BV::from_u64(ctx, lit_up_seg, 7);
        for (word, digit) in itertools::zip_eq(&all_words, &digits) {
            for segment in 0..7 {
                let contains_segment =
                    ast::Bool::from_bool(ctx, word.contains((b'a' + segment) as char));
                for target_segment in 0..7 {
                    solver.assert(
                        &z3::ast::Bool::<'_>::and(
                            ctx,
                            &[
                                &i_as_bv._eq(digit),
                                &mapping[segment as usize]
                                    .extract(target_segment, target_segment)
                                    ._eq(&one),
                            ],
                        )
                        .implies(
                            &lit_up
                                .extract(target_segment.into(), target_segment.into())
                                ._eq(&one)
                            ._eq(&contains_segment),
                        ),
                    );
                }
            }
        }
    }

    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            println!(
                "mapping {}",
                mapping
                    .iter()
                    .map(
                        |bv| (7 - model.eval(bv, true).unwrap().as_i64().unwrap().log2())
                            .to_string()
                            + " "
                    )
                    .collect::<String>()
            );
            return digits
                .iter()
                .skip(examples.len())
                .map(|bv| {
                    (9-model.eval(bv, true).unwrap().as_i64().unwrap().log2()).to_string()
                })
                .collect();
        }
        z3::SatResult::Unsat => {
            panic!("Unsolveable!");
        }
        z3::SatResult::Unknown => {
            panic!("Unknown solvability!");
        }
    }
}
#[test]
fn part1() {
    let mut result = 0;
    let digits = load_digits();
    for (examples, query) in digits {
        let decoded = decode(&examples, &query);
        println!("decoded {}", decoded);
        for c in decoded.chars() {
            if "1478".contains(c) {
                result += 1;
            }
        }
    }
    println!("Digit count is {}", result);
}

#[test]
fn part2() {
    let mut result = 0;
    let digits = load_digits();
    for (examples, query) in digits {
        let decoded = decode(&examples, &query);
        println!("decoded {}", decoded);
        result += decoded.parse::<i32>().unwrap();
    }
    println!("Sum of results is {}", result);
}

