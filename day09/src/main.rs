use std::env;
use std::str::FromStr;
use aocrs::input::get_text;
use crate::disk_map::DiskMap;

mod disk_map;

fn get_input(filename: &str) -> DiskMap {
    DiskMap::from_str(&get_text(filename).unwrap()).unwrap()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Missing input file");
        return;
    }

    let disk = get_input(&args[1]);
    println!("part 1: {}", disk.bit_move_checksum());
    println!("part 2: {}", disk.file_move_checksum());
}
