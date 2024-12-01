use std::str::FromStr;
use aocrs::input::get_text;
use crate::machine::Machine;

mod machine;

fn main() {
    let input = get_text("./day13/input.txt").unwrap().replace("\r\n", "\n");
    let machines: Vec<_> = input
        .split("\n\n")
        .filter_map(|x| Machine::from_str(x).ok())
        .collect();

    let win_tokens: i64 = machines.iter().filter_map(|x| x.win_tokens(0)).sum();
    println!("part 1: {}", win_tokens);     // 480

    let win_tokens: i64 = machines.iter().filter_map(|x| x.win_tokens(10000000000000)).sum();
    println!("part 2: {}", win_tokens);     // 875318608908
}
