use std::str::FromStr;
use aocrs::input::get_text;
use crate::patterns::Patterns;

mod patterns;

fn main() {
    let input = get_text("./day19/input.txt").unwrap();
    let patterns = Patterns::from_str(&input.trim().replace("\r\n", "\n")).unwrap();
    println!("part 1: {}", patterns.possible());        // 6
    println!("part 2: {}", patterns.all_ways());        // 16
}
