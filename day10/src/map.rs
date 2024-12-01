use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use aocrs::geo::direction::Direction;
use aocrs::geo::grid::{CharGrid, ParseGridError};
use aocrs::geo::pos::PosIdx;

const MAX_HEIGHT: u8 = b'9';

#[derive(Clone, Debug)]
pub struct Map {
    grid: CharGrid,
    start_pos: Vec<PosIdx>,
}

impl FromStr for Map {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = CharGrid::from_str(s)?;
        let start_pos: Vec<_> = grid.find_all(b'0').collect();
        Ok(Self { grid, start_pos })
    }
}

impl Map {
    pub fn sum_scores(&self) -> usize {
        self.start_pos.iter().map(|x| self.score_at(x)).sum()
    }

    pub fn sum_ranks(&self) -> usize {
        self.start_pos.iter().map(|x| self.rank_at(x)).sum()
    }

    fn score_at(&self, pos: &PosIdx) -> usize {
        let mut score = 0;
        let mut visited: HashSet<PosIdx> = HashSet::with_capacity(self.grid.size());
        let mut queue: VecDeque<PosIdx> = VecDeque::from(vec![*pos]);

        while let Some(pos) = queue.pop_front() {
            if !visited.insert(pos) {
                continue;
            }

            if self.grid[pos] == MAX_HEIGHT {
                score += 1;
            } else {
                self.valid_neighbours(&pos).iter().for_each(|&x| queue.push_back(x));
            }
        }
        score
    }

    fn rank_at(&self, pos: &PosIdx) -> usize {
        let mut rank = 0;
        let mut stack: Vec<PosIdx> = Vec::with_capacity(self.grid.size());
        stack.push(*pos);

        while let Some(pos) = stack.pop() {
            if self.grid[pos] == MAX_HEIGHT {
                rank += 1;
            } else {
                self.valid_neighbours(&pos).iter().for_each(|&x| stack.push(x));
            }
        }
        rank
    }

    fn valid_neighbours(&self, pos: &PosIdx) -> Vec<PosIdx> {
        let mut valid = Vec::with_capacity(4);
        for dir in Direction::cross() {
            let neighbour = match pos.checked_dest(1, dir) {
                Some(neighbour) if self.grid.has(&neighbour) => neighbour,
                Some(_) | None => continue,
            };
            if self.grid[neighbour] != self.grid[*pos] + 1 {
                continue;
            }
            valid.push(neighbour);
        }
        valid
    }
}
