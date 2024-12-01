use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use aocrs::geo::direction::Direction;
use aocrs::geo::grid::{CharGrid, ParseGridError};
use aocrs::geo::pos::PosIdx;

#[derive(Clone, Debug)]
pub struct Garden(CharGrid);

impl FromStr for Garden {
    type Err = ParseGridError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(CharGrid::from_str(s)?))
    }
}

impl Garden {
    pub fn get_cost(&self, perimeter: bool) -> usize {
        let mut visited: HashSet<PosIdx> = HashSet::with_capacity(self.0.size());
        let mut result = 0;
        for pos in self.0.area() {
            if visited.contains(&pos) {
                continue
            }

            if perimeter {
                result += self.get_area_cost_via_perimeter(pos, &mut visited);
            } else {
                result += self.get_cost_via_sides(pos, &mut visited);
            }
        }
        result
    }

    fn get_area_cost_via_perimeter(&self, pos: PosIdx, visited: &mut HashSet<PosIdx>) -> usize {
        let mut area = 0;
        let mut perimeter = 0;
        let mut queue = VecDeque::from(vec![pos]);

        while let Some(pos) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }

            area += 1;
            let neighbours = self.neighbours(pos, self.0[pos]);
            neighbours.iter().for_each(|&pos| queue.push_back(pos));
            visited.insert(pos);
            perimeter += 4 - neighbours.len();
        }
        area * perimeter
    }

    fn get_cost_via_sides(&self, pos: PosIdx, visited: &mut HashSet<PosIdx>) -> usize {
        let mut area = 0;
        let mut corners = 0;
        let mut queue = VecDeque::from(vec![pos]);

        while let Some(pos) = queue.pop_front() {
            if visited.contains(&pos) {
                continue;
            }

            area += 1;
            let neighbours = self.neighbours(pos, self.0[pos]);
            neighbours.iter().for_each(|&pos| queue.push_back(pos));
            visited.insert(pos);
            corners += self.count_corners(&pos);
        }
        area * corners
    }

    #[inline]
    fn neighbours(&self, pos: PosIdx, plant: u8) -> Vec<PosIdx> {
        self.0
            .area()
            .neighbours(&pos, 1, Direction::cross())
            .filter(|&pos| self.0[pos] == plant)
            .collect()
    }

    fn count_corners(&self, pos: &PosIdx) -> usize {
        let mut count = 0;
        count += if self.is_top_left_corner(pos) { 1 } else { 0 };
        count += if self.is_top_right_corner(pos) { 1 } else { 0 };
        count += if self.is_bottom_left_corner(pos) { 1 } else { 0 };
        count += if self.is_bottom_right_corner(pos) { 1 } else { 0 };
        count
    }

    fn is_top_left_corner(&self, pos: &PosIdx) -> bool {
        let current = Some(self.0[*pos]);
        let up = self.0.destination(pos, 1, Direction::Up);
        let left = self.0.destination(pos, 1, Direction::Left);
        if current == up && current == left {
            current != self.0.destination(pos, 1, Direction::TopLeft)
        } else {
            current != up && current != left
        }
    }

    fn is_top_right_corner(&self, pos: &PosIdx) -> bool {
        let current = Some(self.0[*pos]);
        let up = self.0.destination(pos, 1, Direction::Up);
        let right = self.0.destination(pos, 1, Direction::Right);
        if current == up && current == right {
            current != self.0.destination(pos, 1, Direction::TopRight)
        } else {
            current != up && current != right
        }
    }

    fn is_bottom_left_corner(&self, pos: &PosIdx) -> bool {
        let current = Some(self.0[*pos]);
        let down = self.0.destination(pos, 1, Direction::Down);
        let left = self.0.destination(pos, 1, Direction::Left);
        if current == down && current == left {
            current != self.0.destination(pos, 1, Direction::BottomLeft)
        } else {
            current != down && current != left
        }
    }

    fn is_bottom_right_corner(&self, pos: &PosIdx) -> bool {
        let current = Some(self.0[*pos]);
        let down = self.0.destination(pos, 1, Direction::Down);
        let right = self.0.destination(pos, 1, Direction::Right);
        if current == down && current == right {
            current != self.0.destination(pos, 1, Direction::BottomRight)
        } else {
            current != down && current != right
        }
    }
}
