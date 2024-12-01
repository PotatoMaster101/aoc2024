use std::env;
use std::str::FromStr;
use aocrs::input::get_lines;
use crate::report::Report;

mod report;

fn get_input(filename: &str) -> Vec<Report> {
    let lines = get_lines(filename).unwrap();
    let mut result: Vec<Report> = Vec::with_capacity(1000);
    for line in lines.map_while(Result::ok) {
        result.push(Report::from_str(&line).unwrap());
    }
    result
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Missing input file");
        return;
    }

    let input = get_input(&args[1]);
    let result = input
        .iter()
        .filter(|&report| report.is_safe())
        .count();
    println!("part 1: {}", result);

    let result = input
        .iter()
        .filter(|&report| report.is_dampen_safe())
        .count();
    println!("part 2: {}", result);
}
