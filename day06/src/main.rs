use std::env;
use std::str::FromStr;
use aocrs::input::get_text;
use crate::map::Map;

mod map;

fn get_input(filename: &str) -> Map {
    let input = get_text(filename).unwrap();
    Map::from_str(&input).unwrap()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Missing input file");
        return;
    }

    let map = get_input(&args[1]);
    println!("part 1: {}", map.get_path().len());
    println!("part 2: {}", map.count_blocks());
}
