use std::str::FromStr;
use aocrs::geo::direction::Direction;
use aocrs::geo::grid::{CharGrid, ParseGridError};
use aocrs::geo::pos::PosIdx;

#[derive(Clone, Debug)]
pub struct WordSearch(CharGrid);

impl FromStr for WordSearch {
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(CharGrid::from_str(s)?))
    }
}

impl WordSearch {
    #[inline]
    pub fn count_xmas(&self) -> usize {
        self.0.area().into_iter().filter(|&x| self.0[x] == b'X').map(|x| self.count_at_pos(&x)).sum()
    }

    pub fn count_diag_xmas(&self) -> usize {
        let area = self.0.area();
        let area_filter = area.into_iter().filter(|&x| self.0[x] == b'A' && !area.on_boundary(&x));
        let mut result = 0;
        for pos in area_filter {
            let positions = vec![pos.dest(1, Direction::TopLeft), pos.dest(1, Direction::BottomRight)];
            let str = self.0.extract_string(positions).unwrap();
            if str != "MS" && str != "SM" {
                continue;
            }

            let positions = vec![pos.dest(1, Direction::TopRight), pos.dest(1, Direction::BottomLeft)];
            let str = self.0.extract_string(positions).unwrap();
            if str != "MS" && str != "SM" {
                continue;
            }

            result += 1;
        }
        result
    }

    #[inline]
    fn count_at_pos(&self, pos: &PosIdx) -> usize {
        Direction::all().iter().filter(|&&x| self.is_direction_xmas(pos, x)).count()
    }

    fn is_direction_xmas(&self, pos: &PosIdx, direction: Direction) -> bool {
        let valid: Vec<_> = [
            pos.checked_dest(1, direction),
            pos.checked_dest(2, direction),
            pos.checked_dest(3, direction),
        ].iter().filter_map(|&x| x).collect();
        self.0.extract_string(valid).unwrap() == "MAS"
    }
}
