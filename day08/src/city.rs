use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use aocrs::geo::grid::{CharGrid, ParseGridError};
use aocrs::geo::pos::PosIdx;

#[derive(Clone, Debug)]
pub struct City {
    grid: CharGrid,
    antennas: HashMap<u8, Vec<PosIdx>>,
}

impl FromStr for City {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = CharGrid::from_str(s)?;
        let mut antennas: HashMap<u8, Vec<PosIdx>> = HashMap::with_capacity(26 + 26 + 10);
        for pos in grid.area() {
            if grid[pos] != b'.' {
                antennas.entry(grid[pos]).or_default().push(pos);
            }
        }
        Ok(Self { grid, antennas})
    }
}

impl City {
    pub fn get_all_antinodes(&self) -> HashSet<PosIdx> {
        let mut result = HashSet::with_capacity(500);
        for positions in self.antennas.values() {
            result.extend(self.get_antinodes(positions));
        }
        result
    }

    pub fn get_all_harmonic_antinodes(&self) -> HashSet<PosIdx> {
        let mut result = HashSet::with_capacity(500);
        for positions in self.antennas.values() {
            result.extend(self.get_harmonic_antinodes(positions));
        }
        result
    }

    fn get_antinodes(&self, positions: &[PosIdx]) -> HashSet<PosIdx> {
        let mut result = HashSet::new();
        for i in 1..positions.len() {
            for j in 0..i {
                if let Some(val) = self.get_antinode(positions[i], positions[j]) {
                    result.insert(val);
                }
                if let Some(val) = self.get_antinode(positions[j], positions[i]) {
                    result.insert(val);
                }
            }
        }
        result
    }

    fn get_antinode(&self, source: PosIdx, other: PosIdx) -> Option<PosIdx> {
        let x_diff = source.x.abs_diff(other.x);
        let x = if source.x < other.x {
            source.x.checked_sub(x_diff)?
        } else {
            source.x + x_diff
        };

        let y_diff = source.y.abs_diff(other.y);
        let y = if source.y < other.y {
            source.y.checked_sub(y_diff)?
        } else {
            source.y + y_diff
        };

        let result = PosIdx::new(x, y);
        if self.grid.has(&result) {
            Some(result)
        } else {
            None
        }
    }

    fn get_harmonic_antinodes(&self, positions: &[PosIdx]) -> HashSet<PosIdx> {
        let mut result = HashSet::new();
        for i in 1..positions.len() {
            for j in 0..i {
                result.extend([positions[i], positions[j]]);
                result.extend(self.get_harmonic_antinode(positions[i], positions[j]));
                result.extend(self.get_harmonic_antinode(positions[j], positions[i]));
            }
        }
        result
    }

    fn get_harmonic_antinode(&self, source: PosIdx, other: PosIdx) -> Vec<PosIdx> {
        let mut result = Vec::new();
        let mut previous = source;
        let mut current = other;
        while let Some(next) = self.get_antinode(current, previous) {
            if !self.grid.has(&next) {
                break;
            }

            result.push(next);
            previous = current;
            current = next;
        }
        result
    }
}
