use std::str::FromStr;
use aocrs::geo::direction::Direction;
use aocrs::geo::grid::{CharGrid, ParseGridError};
use aocrs::geo::pos::PosIdx;

const AIR: u8 = b'.';
const BOX: u8 = b'O';
const BOX_LEFT: u8 = b'[';
const BOX_RIGHT: u8 = b']';
const ROBOT: u8 = b'@';
const WALL: u8 = b'#';

#[derive(Clone, Debug)]
pub struct Map {
    grid: CharGrid,
    moves: Vec<u8>,
    start_pos: PosIdx,
}

impl FromStr for Map {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split("\n\n").collect();
        let grid = CharGrid::from_str(splits[0])?;
        let moves: Vec<_> = splits[1].replace("\n", "").into_bytes();
        let start_pos = grid.find(b'@').unwrap();
        Ok(Map { grid, moves, start_pos })
    }
}

impl Map {
    pub fn gps(&self) -> usize {
        let mut grid = self.grid.clone();
        let mut pos = self.start_pos;
        for &m in &self.moves {
            let mut dir = Direction::from(m);
            if dir == Direction::Up || dir == Direction::Down {
                dir = dir.back();
            }
            pos = move_dir(&mut grid, pos, dir);
        }

        let mut result = 0;
        for pos in grid.area() {
            if grid[pos] == BOX {
                result += 100 * pos.y + pos.x;
            }
        }
        result
    }

    pub fn big_gps(&self) -> usize {
        let mut grid = expand_map(&self.grid);
        0
    }
}

fn expand_map(grid: &CharGrid) -> CharGrid {
    let mut data: Vec<u8> = Vec::with_capacity(grid.width * 2 * grid.height);
    let area = grid.area();
    for pos in area {
        if grid[pos] == WALL {
            data.extend([WALL; 2].iter());
        } else if grid[pos] == BOX {
            data.extend([BOX_LEFT, BOX_RIGHT].iter());
        } else if grid[pos] == ROBOT {
            data.extend([ROBOT, AIR].iter());
        } else {
            data.extend([AIR; 2].iter());
        }
    }
    CharGrid::with_data(grid.width * 2, &data).unwrap()
}

fn move_dir(grid: &mut CharGrid, pos: PosIdx, dir: Direction) -> PosIdx {
    let air = match next_air(grid, pos, dir) {
        Some(x) => x,
        None => return pos,
    };

    let new_pos = pos.dest(1, dir);
    grid[air] = BOX;
    grid[pos] = AIR;
    grid[new_pos] = ROBOT;
    new_pos
}

fn next_air(grid: &CharGrid, pos: PosIdx, dir: Direction) -> Option<PosIdx> {
    let mut dist = 1;
    while let Some(dest) = pos.checked_dest(dist, dir) {
        if grid[dest] == WALL {
            return None;
        }
        if grid[dest] == AIR {
            return Some(dest);
        }
        dist += 1;
    }
    None
}

fn print_map(grid: &CharGrid) {
    for pos in grid.area() {
        if pos.x == 0 {
            println!();
        }
        print!("{}", grid[pos] as char);
    }
    println!();
}
