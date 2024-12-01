use std::str::FromStr;
use aocrs::input::get_text;
use crate::disk_map::DiskMap;

mod disk_map;

fn main() {
    let disk = DiskMap::from_str(&get_text("./day09/input.txt").unwrap()).unwrap();
    println!("part 1: {}", disk.bit_move_checksum());       // 1928
    println!("part 2: {}", disk.file_move_checksum());      // 2858
}
