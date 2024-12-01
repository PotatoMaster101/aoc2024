use std::str::FromStr;
use aocrs::input::get_text;
use crate::garden::Garden;

mod garden;

fn main() {
    let garden = Garden::from_str(&get_text("./day12/input.txt").unwrap()).unwrap();
    println!("part 1: {}", garden.get_cost(true));      // 140
    println!("part 2: {}", garden.get_cost(false));     // 80
}
