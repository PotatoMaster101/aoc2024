use std::str::FromStr;
use aocrs::input::get_text;
use crate::debugger::Debugger;

mod debugger;

fn main() {
    let mut debugger = Debugger::from_str(&get_text("./day17/input.txt").unwrap()).unwrap();
    let reverse = debugger.reverse();
    let results: Vec<_> = debugger.run()
        .iter()
        .map(|x| x.to_string())
        .collect();

    println!("part 1: {}", results.join(","));      // 5,7,3,0
    println!("part 2: {}", reverse);                    // 117440   TODO
}
