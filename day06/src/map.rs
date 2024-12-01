use std::collections::HashSet;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use aocrs::geo::area::Area;
use aocrs::geo::direction::{Direction, DirectionalPos};
use aocrs::geo::grid::{CharGrid, ParseGridError};
use aocrs::geo::pos::PosIdx;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone, Debug)]
pub struct Map {
    grid: CharGrid,
    start: PosIdx,
}

impl FromStr for Map {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = CharGrid::from_str(s)?;
        let start = grid.find(b'^').unwrap();
        Ok(Self { grid, start })
    }
}

impl Map {
    pub fn get_path(&self) -> HashSet<PosIdx> {
        let mut visited: HashSet<PosIdx> = HashSet::with_capacity(7500);
        self.iterate_path(|x| { visited.insert(x.pos); });
        visited
    }

    pub fn count_blocks(&self) -> usize {
        let count = AtomicUsize::new(0);
        let area = self.grid.area();
        let path = self.get_path();
        path.into_par_iter().for_each(|pos| {
            if pos == self.start {
                return;
            }

            if self.is_loop(pos, area) {
                count.fetch_add(1, Ordering::SeqCst);
            }
        });
        count.load(Ordering::SeqCst)
    }

    fn is_loop(&self, block: PosIdx, area: Area<usize>) -> bool {
        let mut dpos = DirectionalPos::new(self.start, Direction::Down);
        let mut visited: HashSet<DirectionalPos<usize>> = HashSet::with_capacity(7500);
        while !area.on_boundary(&dpos.pos) {
            let next = dpos.next(1);
            if next.pos == block || self.grid[next.pos] == b'#' {
                dpos = dpos.update_direction(dpos.direction.left());
            } else {
                dpos = next;
            }

            if !visited.insert(dpos) {
                return true;
            }
        }
        false
    }

    fn iterate_path(&self, mut on_each: impl FnMut(DirectionalPos<usize>)) {
        // grid is reversed, inverse direction
        let mut dpos = DirectionalPos::new(self.start, Direction::Down);
        on_each(dpos);

        let area = self.grid.area();
        while !area.on_boundary(&dpos.pos) {
            let next = dpos.next(1);
            if self.grid[next.pos] == b'#' {
                dpos = dpos.update_direction(dpos.direction.left());
            } else {
                dpos = next;
            }
            on_each(dpos);
        }
    }
}
