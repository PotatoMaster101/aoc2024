use std::str::FromStr;
use aocrs::input::get_text;
use crate::city::City;

mod city;

fn main() {
    let city = City::from_str(&get_text("./day08/input.txt").unwrap()).unwrap();
    println!("part 1: {}", city.get_all_antinodes().len());             // 14
    println!("part 2: {}", city.get_all_harmonic_antinodes().len());    // 34
}
