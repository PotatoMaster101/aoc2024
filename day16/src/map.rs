use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;
use aocrs::geo::direction::{Direction, DirectionalPos};
use aocrs::geo::grid::{CharGrid, ParseGridError};
use aocrs::geo::pos::PosIdx;
use crate::state::State;

const END: u8 = b'E';
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
    pub fn score(&self) -> usize {
        self.dijkstra()[0].cost
    }

    pub fn seats(&self) -> usize {
        let mut paths = HashSet::new();
        for state in self.dijkstra() {
            paths.extend(state.path);
        }
        paths.len()
    }

    fn dijkstra(&self) -> Vec<State> {
        let mut processed = HashSet::with_capacity(self.grid.size());
        let mut pq = BinaryHeap::new();
        processed.insert(DirectionalPos::new(self.start, Direction::Right));
        pq.push(Reverse(State::new(0, DirectionalPos::new(self.start, Direction::Right), HashSet::new())));

        let mut best_cost = usize::MAX;
        let mut states = Vec::new();
        while let Some(Reverse(state)) = pq.pop() {
            processed.insert(state.dpos);
            if self.grid[state.dpos.pos] == END {
                if state.cost <= best_cost {
                    best_cost = state.cost;
                    states.push(state.clone());
                } else if state.cost > best_cost {
                    break;
                }
            }

            state.neighbours()
                .into_iter()
                .filter(|n| !processed.contains(&n.dpos) && self.grid[n.dpos.pos] != WALL)
                .for_each(|n| pq.push(Reverse(n)));
        }
        states
    }
}
