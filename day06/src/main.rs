use std::str::FromStr;
use aocrs::input::get_text;
use crate::map::Map;

mod map;

fn main() {
    let map =     Map::from_str(&get_text("./day06/input.txt").unwrap()).unwrap();
    println!("part 1: {}", map.get_path().len());       // 41
    println!("part 2: {}", map.count_blocks());         // 6
}
