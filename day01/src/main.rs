use std::collections::HashMap;
use aocrs::input::get_lines;
use itertools::Itertools;

fn get_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = get_lines(filename).unwrap();
    let mut first = Vec::with_capacity(1000);
    let mut second = Vec::with_capacity(1000);
    for line in lines.map_while(Result::ok) {
        let splits: Vec<_> = line.split("   ").collect();
        first.push(splits[0].parse::<i32>().unwrap());
        second.push(splits[1].parse::<i32>().unwrap());
    }

    first.sort();
    second.sort();
    (first, second)
}

fn main() {
    let (first, second) = get_input("./day01/input.txt");
    let mut result = 0;
    for idx in 0..first.len() {
        result += (first[idx] - second[idx]).abs();
    }
    println!("part 1: {}", result);     // 11

    result = 0;
    let occurrences: HashMap<_, _> = second
        .iter()
        .into_grouping_map_by(|&&v| v)
        .fold(0, |acc, _, _| acc + 1);
    for num in first {
        let occurrence = occurrences.get(&num);
        result += match occurrence {
            Some(v) => num * v,
            None => 0,
        };
    }
    println!("part 2: {}", result);     // 31
}
