use std::str::FromStr;
use aocrs::geo::grid::{CharGrid, GridParseError};
use aocrs::geo::pos::PosIdx;

#[derive(Debug, Clone)]
pub struct Map {
    grid: CharGrid,
    start_pos: Vec<PosIdx>,
}

impl FromStr for Map {
    type Err = GridParseError;

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

    fn score_at(&self, pos: &PosIdx) -> usize {
        0
    }
}
