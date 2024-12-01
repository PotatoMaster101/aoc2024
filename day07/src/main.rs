use std::env;
use std::str::FromStr;
use aocrs::input::get_lines;
use crate::equation::Equation;

mod equation;

fn get_input(filename: &str) -> Vec<Equation> {
    let mut result: Vec<Equation> = Vec::with_capacity(1000);
    let lines = get_lines(filename).unwrap();
    for line in lines.map_while(Result::ok) {
        result.push(Equation::from_str(&line).unwrap());
    }
    result
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Missing input file");
        return;
    }

    let equations = get_input(&args[1]);
    let passed_sum: i64 = equations
        .iter()
        .map(|x| x.get_ans(b"*+"))
        .sum();
    println!("part 1: {}", passed_sum);

    let passed_sum: i64 = equations
        .iter()
        .map(|x| x.get_ans(b"*+|"))
        .sum();
    println!("part 2: {}", passed_sum);
}
