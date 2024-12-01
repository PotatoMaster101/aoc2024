use std::str::FromStr;
use aocrs::input::get_text;
use crate::map::Map;

mod map;

fn main() {
    let map = Map::from_str(&get_text("./day10/input.txt").unwrap()).unwrap();
    println!("part 1: {}", map.sum_scores());       // 36
    println!("part 2: {}", map.sum_ranks());        // 81
}
