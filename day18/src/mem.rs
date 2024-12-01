use std::collections::{HashSet, VecDeque};
use std::num::ParseIntError;
use std::str::FromStr;
use aocrs::geo::area::Area;
use aocrs::geo::direction::Direction;
use aocrs::geo::pos::PosIdx;

#[derive(Clone, Debug)]
pub struct Memory {
    corrupted: Vec<PosIdx>,
    start: PosIdx,
    end: PosIdx,
}

impl Memory {
    pub fn load(s: &str, bound: usize) -> Result<Self, ParseIntError> {
        let mut corrupted = Vec::with_capacity(5000);
        let lines: Vec<_> = s.trim().lines().collect();
        for line in lines {
            let nums: Vec<_> = line.split(",").collect();
            corrupted.push(PosIdx::new(usize::from_str(nums[0])?, usize::from_str(nums[1])?));
        }
        Ok(Self { corrupted, start: PosIdx::origin(), end: PosIdx::new(bound, bound) })
    }

    pub fn steps(&self, size: usize) -> usize {
        self.bfs(size).unwrap_or(usize::MAX)
    }

    pub fn block(&self) -> PosIdx {
        let mut min = 0;
        let mut max = self.corrupted.len() - 1;
        while min < max {
            let mid = (min + max) / 2 + 1;
            if self.bfs(mid).is_some() {
                min = mid;
            } else {
                max = mid - 1;
            }
        }
        self.corrupted[min]
    }

    fn bfs(&self, size: usize) -> Option<usize> {
        let area = Area::new(self.end.x, self.end.y, 0, 0).unwrap();
        let corrupted: HashSet<PosIdx> = HashSet::from_iter(self.corrupted.iter().copied().take(size));
        let mut visited: HashSet<PosIdx> = HashSet::with_capacity(area.size());
        let mut queue = VecDeque::from(vec![(self.start, 0)]);
        while let Some((pos, steps)) = queue.pop_front() {
            if !visited.insert(pos) {
                continue;
            }
            if pos == self.end {
                return Some(steps);
            }

            for neighbour in area.neighbours(&pos, 1, Direction::cross()) {
                if !corrupted.contains(&neighbour) {
                    queue.push_back((neighbour, steps + 1));
                }
            }
        }
        None
    }
}
