use std::env;
use std::str::FromStr;
use aocrs::input::get_text;
use crate::word_search::WordSearch;

mod word_search;

fn get_input(filename: &str) -> WordSearch {
    let input = get_text(filename).unwrap();
    WordSearch::from_str(&input).unwrap()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Missing input file");
        return;
    }

    let search = get_input(&args[1]);
    println!("part 1: {}", search.count_xmas());
    println!("part 2: {}", search.count_diag_xmas());
}
