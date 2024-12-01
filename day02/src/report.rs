use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Report(Vec<i32>);

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels: Vec<_> = s
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        Ok(Report(levels))
    }
}

impl Report {
    pub fn is_safe(&self) -> bool {
        Self::check_safe(&self.0, usize::MAX)
    }

    pub fn is_dampen_safe(&self) -> bool {
        for skip in 0..self.0.len() {
            if Self::check_safe(&self.0, skip) {
                return true;
            }
        }
        false
    }

    fn check_safe(nums: &[i32], skip: usize) -> bool {
        let mut pos = HashSet::from([1, 2, 3]);
        let mut neg = HashSet::from([-1, -2, -3]);
        if skip == 0 {
            return Self::check_safe(&nums[1..], usize::MAX);
        }
        if skip == nums.len() - 1 {
            return Self::check_safe(&nums[..nums.len() - 1], usize::MAX);
        }

        for idx in 1..nums.len() {
            if skip == idx {
                continue;
            }

            let current = nums[idx];
            let mut previous = nums[idx - 1];
            if skip != usize::MAX && idx == skip + 1 {
                previous = nums[idx - 2];
            }

            pos.insert(current - previous);
            neg.insert(current - previous);
        }
        pos.len() == 3 || neg.len() == 3
    }
}
