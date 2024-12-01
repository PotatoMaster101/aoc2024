use std::str::FromStr;
use aocrs::input::get_text;
use crate::map::Map;

mod map;

fn main() {
    let input = get_text("./day20/input.txt").unwrap();
    let map = Map::from_str(&input).unwrap();
    println!("part 1: {}", map.cheats(2));      // 0 on example input
    println!("part 2: {}", map.cheats(20));     // 0 on example input
}
