use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::{Mul, Not};
use z3::ast::Ast;

#[derive(Debug)]
enum Arg {
    Var(char),
    Lit(i64),
}

impl Arg {
    fn parse(s: &str) -> Arg {
        match s.parse::<i64>() {
            Ok(val) => Arg::Lit(val),
            Err(_) => {
                assert_eq!(s.len(), 1);
                Arg::Var(s.chars().next().unwrap())
            }
        }
    }
    fn eval<'c>(
        &self,
        vars: &HashMap<char, z3::ast::BV<'c>>,
        ctx: &'c z3::Context,
    ) -> z3::ast::BV<'c> {
        match self {
            Arg::Var(c) => vars.get(c).unwrap().clone(),
            Arg::Lit(v) => z3::ast::BV::from_i64(ctx, *v, BIT_WIDTH),
        }
    }
}

#[derive(Debug)]
enum Op {
    Inp(Arg),
    Add(Arg, Arg),
    Mul(Arg, Arg),
    Div(Arg, Arg),
    Mod(Arg, Arg),
    Eql(Arg, Arg),
}
lazy_static! {
    /// This is an example for using doc comment attributes
    static ref OP_RE: Regex = Regex::new("(inp|add|mul|div|mod|eql) ([^ ]*)( (.*))?").unwrap();
}
static BIT_WIDTH: u32 = 64;

impl Op {
    fn parse(line: &str) -> Op {
        let captures = OP_RE.captures(line).unwrap();
        let arg1 = Arg::parse(captures.get(2).unwrap().as_str());
        let arg2 = captures.get(4).map(|m| Arg::parse(m.as_str()));
        match captures.get(1).unwrap().as_str() {
            "inp" => Op::Inp(arg1),
            "add" => Op::Add(arg1, arg2.unwrap()),
            "mul" => Op::Mul(arg1, arg2.unwrap()),
            "div" => Op::Div(arg1, arg2.unwrap()),
            "mod" => Op::Mod(arg1, arg2.unwrap()),
            "eql" => Op::Eql(arg1, arg2.unwrap()),
            _ => panic!(),
        }
    }

    fn apply<'a, 'c: 'a, I: Iterator<Item = &'a z3::ast::BV<'c>>>(
        &self,
        step: usize,
        input: &mut I,
        vars: &mut HashMap<char, z3::ast::BV<'c>>,
        ctx: &'c z3::Context,
        optimize: &z3::Optimize,
    ) {
        let var_as_int = |c| vars.get(c).unwrap();
        let (c, val): (char, z3::ast::BV) = match self {
            Op::Inp(Arg::Var(c)) => (*c, input.next().unwrap().clone()),
            Op::Add(Arg::Var(c), val) => (*c, var_as_int(c) + val.eval(vars, ctx)),
            Op::Mul(Arg::Var(c), val) => (*c, var_as_int(c) * val.eval(vars, ctx)),
            Op::Div(Arg::Var(c), val) => {
                let den = val.eval(vars, ctx);
                optimize.assert(&den._eq(&z3::ast::BV::from_i64(ctx, 0, BIT_WIDTH)).not());
                (*c, var_as_int(c).bvsdiv(&den))
            }
            Op::Mod(Arg::Var(c), val) => {
                let den = val.eval(vars, ctx);
                optimize.assert(&den._eq(&z3::ast::BV::from_i64(ctx, 0, BIT_WIDTH)).not());
                (*c, var_as_int(c).bvsrem(&den))
            }
            Op::Eql(Arg::Var(c), val) => (
                *c,
                var_as_int(c)._eq(&val.eval(vars, ctx)).ite(
                    &z3::ast::BV::from_i64(ctx, 1, BIT_WIDTH),
                    &z3::ast::BV::from_i64(ctx, 0, BIT_WIDTH),
                ),
            ),
            _ => panic!("{:?}", self),
        };
        let var = z3::ast::BV::new_const(ctx, format!("{}{}", c, step), BIT_WIDTH);
        optimize.assert(&var._eq(&val));
        vars.insert(c, var);
    }
}

fn load_ops() -> Vec<Op> {
    let file = File::open("24.txt").unwrap();
    let lines: io::Lines<io::BufReader<File>> = io::BufReader::new(file).lines();

    return lines.map(|l| Op::parse(l.unwrap().as_str())).collect();
}

fn solve(ops: &Vec<Op>) {
    let z3_conf = z3::Config::new();
    let ctx = &z3::Context::new(&z3_conf);
    let optimize = z3::Optimize::new(&ctx);
    let inputs = (0..14)
        .map(|i| z3::ast::BV::new_const(&ctx, format!("digit_{}", i), BIT_WIDTH))
        .collect_vec();
    for input in inputs.iter() {
        optimize.assert(&input.bvslt(&z3::ast::BV::from_i64(&ctx, 10, BIT_WIDTH)));
        optimize.assert(&input.bvsge(&z3::ast::BV::from_i64(&ctx, 1, BIT_WIDTH)));
    }

    let mut curr_values = HashMap::<char, z3::ast::BV>::new();
    for var in ['w', 'x', 'y', 'z'] {
        curr_values.insert(var, z3::ast::BV::from_i64(&ctx, 0, BIT_WIDTH));
    }
    let mut input_iter = inputs.iter();
    for (step, op) in ops.iter().enumerate() {
        op.apply(step, &mut input_iter, &mut curr_values, &ctx, &optimize);
    }

    optimize.assert(
        &curr_values
            .get(&'z')
            .unwrap()
            ._eq(&z3::ast::BV::from_i64(&ctx, 0, BIT_WIDTH).into()),
    );
    let total_score = inputs
        .iter()
        .cloned()
        .reduce(|left, right| left.mul(10i64) + right)
        .unwrap();
    println!("Attempting to solve...");
    optimize.minimize(&total_score);
    println!("{:?}", &optimize);
    match optimize.check(&[]) {
        z3::SatResult::Unsat => println!("UNSAT"),
        z3::SatResult::Unknown => println!("UNKNOWN!"),
        z3::SatResult::Sat => {
            println!("SAT");
            let model = optimize.get_model().unwrap();
            for input in inputs.iter() {
                print!("{}", model.eval(input, false).unwrap());
            }
            println!();
        }
    }
}
#[test]
fn mini_test() {
    let ops = vec![
        Op::Inp(Arg::Var('z')),
        Op::Inp(Arg::Var('x')),
        Op::Add(Arg::Var('z'), Arg::Var('x')),
        Op::Div(Arg::Var('z'), Arg::Lit(2)),
        Op::Eql(Arg::Var('z'), Arg::Lit(4)),
        Op::Eql(Arg::Var('z'), Arg::Lit(0)),
    ];
    println!("op {:?}", ops);
    solve(&ops);
}

#[test]
fn part1() {
    let mut ops = load_ops();
    println!("op {:?}", ops);
    solve(&ops);
}
