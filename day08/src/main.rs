use std::env;
use std::str::FromStr;
use aocrs::input::get_text;
use crate::city::City;

mod city;

fn get_input(filename: &str) -> City {
    City::from_str(&get_text(filename).unwrap()).unwrap()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Missing input file");
        return;
    }

    let city = get_input(&args[1]);
    println!("part 1: {}", city.get_all_antinodes().len());
    println!("part 2: {}", city.get_all_harmonic_antinodes().len());
}
