use std::str::FromStr;
use aocrs::input::get_text;
use crate::code::Code;

mod code;

fn main() {
    let input = get_text("./day21/input.txt").unwrap();
    let codes: Vec<_> = input.lines().filter_map(|x| Code::from_str(x).ok()).collect();
    let complexity: usize = codes.iter().map(Code::complexity).sum();
    println!("part 1: {}", complexity);
    println!("part 2: {}", 0);
}
