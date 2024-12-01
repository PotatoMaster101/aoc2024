use std::str::FromStr;
use aocrs::input::get_text;
use crate::map::Map;

mod map;

fn main() {
    let input = get_text("./day15/input.txt").unwrap();
    let map = Map::from_str(&input.replace("\r\n", "\n")).unwrap();
    println!("part 1: {}", map.gps());      // 10092
    println!("part 2: {}", map.big_gps());  // 9021     TODO
}
