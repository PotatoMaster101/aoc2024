use std::str::FromStr;
use aocrs::input::get_text;
use crate::map::Map;

mod map;
mod state;

fn main() {
    let map = Map::from_str(&get_text("./day16/input.txt").unwrap()).unwrap();
    println!("part 1: {}", map.score());    // 7036
    println!("part 2: {}", map.seats());    // 45
}
