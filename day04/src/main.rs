use std::str::FromStr;
use aocrs::input::get_text;
use crate::word_search::WordSearch;

mod word_search;

fn main() {
    let search = WordSearch::from_str(&get_text("./day04/input.txt").unwrap()).unwrap();
    println!("part 1: {}", search.count_xmas());            // 18
    println!("part 2: {}", search.count_diag_xmas());       // 9
}
