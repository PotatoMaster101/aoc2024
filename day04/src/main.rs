use std::env;
use std::str::FromStr;
use aocrs::geo::area::Area;
use aocrs::geo::direction::Direction;
use aocrs::geo::grid::CharGrid;
use aocrs::geo::pos::{PosIdx, SignedPosIdx};
use aocrs::input::get_text;

fn get_input(filename: &str) -> CharGrid {
    let line = get_text(filename).unwrap();
    CharGrid::from_str(&line).unwrap()
}

fn count_xmas(grid: &CharGrid, pos: &SignedPosIdx) -> usize {
    let area = grid.signed_area().unwrap();
    let mut count = 0;
    for dir in Direction::all() {
        let line = pos.line_iter(4, dir);
        let valid: Vec<_> = area.filter_pos(line).filter_map(|p| PosIdx::try_from(p).ok()).collect();
        if valid.len() != 4 {
            continue;
        }

        let word = String::from_utf8(vec![grid[valid[0]], grid[valid[1]], grid[valid[2]], grid[valid[3]]]).unwrap();
        if word == "XMAS" {
            count += 1;
        }
    }
    count
}

fn count_xmas_diag(grid: &CharGrid) -> usize {
    let area = grid.area();
    let borderless = Area::from_pos(area.top_left().bottom_right(1), area.bottom_right().top_left(1)).unwrap();
    let mut count = 0;

    for pos in borderless {
        if grid[pos] != b'A' {
            continue;
        }

        // check top left and bottom right
        let diag = String::from_utf8(vec![grid[pos.top_left(1)], grid[pos.bottom_right(1)]]).unwrap();
        if diag != "MS" && diag != "SM" {
            continue;
        }

        // check top right and bottom left
        let diag = String::from_utf8(vec![grid[pos.top_right(1)], grid[pos.bottom_left(1)]]).unwrap();
        if diag != "MS" && diag != "SM" {
            continue;
        }

        count += 1;
    }
    count
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Missing input file");
        return;
    }

    let input = get_input(&args[1]);
    let mut result = 0;
    for pos in input.signed_area().unwrap() {
        result += count_xmas(&input, &pos);
    }
    println!("part 1: {}", result);
    println!("part 2: {}", count_xmas_diag(&input));
}
