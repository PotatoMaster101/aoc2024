use std::num::ParseIntError;
use std::str::FromStr;
use aocrs::math::digits;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Equation {
    ans: i64,
    nums: Vec<i64>,
}

impl FromStr for Equation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = s.split(": ").collect();
        let ans: i64 = i64::from_str(splits[0])?;
        let nums: Vec<_> = splits[1]
            .split(" ")
            .filter_map(|n| i64::from_str(n).ok())
            .collect();
        Ok(Self { ans, nums })
    }
}

impl Equation {
    pub fn get_ans(&self, opr: &[u8]) -> i64 {
        let perms = (0..self.nums.len() - 1)
            .map(|_| opr.iter())
            .multi_cartesian_product();

        for perm in perms {
            let perm: Vec<_> = perm.into_iter().copied().collect();
            if self.is_solution(&perm) {
                return self.ans;
            }
        }
        0
    }

    fn is_solution(&self, perm: &[u8]) -> bool {
        let mut result = self.nums[0];
        for (idx, opr) in perm.iter().enumerate() {
            if *opr == b'*' {
                result *= self.nums[idx + 1];
            } else if *opr == b'+' {
                result += self.nums[idx + 1];
            } else if *opr == b'|' {
                result = result * 10i64.pow(digits(self.nums[idx + 1])) + self.nums[idx + 1];
            }
        }
        result == self.ans
    }
}
