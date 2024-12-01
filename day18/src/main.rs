use aocrs::input::get_text;
use crate::mem::Memory;

mod mem;

fn main() {
    let input = get_text("./day18/input.txt").unwrap();
    let memory = Memory::load(&input, 70).unwrap();
    println!("part 1: {}", memory.steps(1024));     // 146

    let block = memory.block();
    println!("part 2: {},{}", block.x, block.y);        // 2,0
}
