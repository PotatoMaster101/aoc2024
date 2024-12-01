use std::str::FromStr;
use crate::page_order::PageOrderMap;

#[derive(Clone, Debug)]
pub struct Update(Vec<i32>);

impl FromStr for Update {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<_> = s.split(",").filter_map(|n| n.parse::<i32>().ok()).collect();
        Ok(Self(nums))
    }
}

impl Update {
    pub fn is_valid(&self, map: &PageOrderMap) -> bool {
        for after_idx in 1..self.0.len() {
            for before_idx in 0..after_idx {
                if !map.is_valid(self.0[before_idx], self.0[after_idx]) {
                    return false;
                }
            }
        }
        true
    }

    #[inline]
    pub fn middle(&self) -> i32 {
        self.0[self.0.len() / 2]
    }

    pub fn fix(&self, map: &PageOrderMap) -> Update {
        let mut result = self.0.clone();
        for after_idx in 1..result.len() {
            for before_idx in 0..after_idx {
                if !map.is_valid(result[before_idx], result[after_idx]) {
                    result.swap(before_idx, after_idx);
                }
            }
        }
        Update(result)
    }
}
