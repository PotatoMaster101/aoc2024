use std::str::FromStr;
use aocrs::geo::area::Area;
use aocrs::input::get_all_lines;
use crate::robot::{find_tree, Quadrant, Robot};

mod robot;

fn main() {
    let area = Area::new(101, 103, 0, 0).unwrap();
    let robots: Vec<_> = get_all_lines("./day14/input.txt")
        .unwrap()
        .iter()
        .filter_map(|x| Robot::from_str(x).ok())
        .collect();

    let (mut top_left, mut top_right, mut bottom_left, mut bottom_right) = (0, 0, 0, 0);
    for robot in &robots {
        match robot.quadrant(&area, 100) {
            Quadrant::TopLeft => top_left += 1,
            Quadrant::TopRight => top_right += 1,
            Quadrant::BottomLeft => bottom_left += 1,
            Quadrant::BottomRight => bottom_right += 1,
            _ => ()
        }
    }
    println!("part 1: {}", top_left * top_right * bottom_left * bottom_right);      // 21
    println!("part 2: {}", find_tree(&robots, &area));          // no tree in example input
}
