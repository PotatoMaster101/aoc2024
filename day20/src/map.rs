use std::collections::VecDeque;
use std::str::FromStr;
use aocrs::geo::direction::Direction;
use aocrs::geo::grid::{CharGrid, ParseGridError};
use aocrs::geo::pos::PosIdx;

const MAX_DIST: usize = usize::MAX;
const END: u8 = b'E';
const START: u8 = b'S';
const WALL: u8 = b'#';

#[derive(Clone, Debug)]
pub struct Map {
    grid: CharGrid,
    start: PosIdx,
    end: PosIdx,
}

impl FromStr for Map {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = CharGrid::from_str(s)?;
        let start = grid.find(START).unwrap();
        let end = grid.find(END).unwrap();
        Ok(Self { grid, start, end })
    }
}

impl Map {
    pub fn cheats(&self, min: i64) -> usize {
        let dists = self.distances();
        let mut count = 0;
        for pos in self.grid.area().into_iter().filter(|&p| self.grid[p] != WALL) {
            count += self.grid.area()
                .neighbours(&pos, 2, Direction::cross())
                .filter(|&n| self.grid[n] != WALL && (dists[pos.y][pos.x] as i64) - (dists[n.y][n.x] as i64) >= 100 + min)
                .count();
        }
        count
    }

    fn distances(&self) -> Vec<Vec<usize>> {
        let area = self.grid.area();
        let mut queue = VecDeque::from([self.start]);
        let mut dists = vec![vec![MAX_DIST; self.grid.width]; self.grid.height];
        dists[self.start.y][self.start.x] = 0;
        while let Some(pos) = queue.pop_front() {
            let neighbours = area.neighbours(&pos, 1, Direction::cross()).filter(|&pos| self.grid[pos] != WALL);
            for neighbour in neighbours {
                if dists[neighbour.y][neighbour.x] == MAX_DIST {
                    dists[neighbour.y][neighbour.x] = dists[pos.y][pos.x] + 1;
                    queue.push_back(neighbour);
                }
            }
        }
        dists
    }
}
