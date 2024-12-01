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
    let input = get_input("./day02/input.txt");
    let result = input
        .iter()
        .filter(|&report| report.is_safe())
        .count();
    println!("part 1: {}", result);     // 2

    let result = input
        .iter()
        .filter(|&report| report.is_dampen_safe())
        .count();
    println!("part 2: {}", result);     // 4
}
