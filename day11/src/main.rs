use std::str::FromStr;
use aocrs::input::get_text;
use crate::stones::Stones;

mod stones;

fn main() {
    let stones = Stones::from_str(&get_text("./day11/input.txt").unwrap()).unwrap();
    println!("part 1: {}", stones.count(25));       // 55312
    println!("part 2: {}", stones.count(75));       // 65601038650482
}
