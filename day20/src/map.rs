use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use aocrs::geo::direction::Direction;
use aocrs::geo::grid::{CharGrid, ParseGridError};
use aocrs::geo::pos::PosIdx;

const START: u8 = b'S';
const WALL: u8 = b'#';

#[derive(Clone, Debug)]
pub struct Map {
    grid: CharGrid,
    start: PosIdx,
}

impl FromStr for Map {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = CharGrid::from_str(s)?;
        let start = grid.find(START).unwrap();
        Ok(Self { grid, start })
    }
}

impl Map {
    pub fn cheats(&self, max_dist: usize) -> usize {
        let dists: Vec<_> = self.map_dist().into_iter().collect();
        let mut count = 0;
        for curr in 1..dists.len() {
            for prev in 0..curr {
                let manhattan = dists[curr].0.manhattan_unsigned(&dists[prev].0);
                if manhattan <= max_dist && dists[curr].1.abs_diff(dists[prev].1) - manhattan >= 100 {
                    count += 1;
                }
            }
        }
        count
    }

    fn map_dist(&self) -> HashMap<PosIdx, usize> {
        let area = self.grid.area();
        let mut queue = VecDeque::from([self.start]);
        let mut result = HashMap::with_capacity(10000);
        result.insert(self.start, 0);
        while let Some(pos) = queue.pop_front() {
            let neighbours = area
                .neighbours(&pos, 1, Direction::cross())
                .filter(|&pos| self.grid[pos] != WALL);

            for neighbour in neighbours {
                if !result.contains_key(&neighbour) {
                    queue.push_back(neighbour);
                    result.insert(neighbour, result[&pos] + 1);
                }
            }
        }
        result
    }
}
