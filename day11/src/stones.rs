use std::collections::HashMap;
use std::str::FromStr;
use aocrs::math::digits;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct CacheEntry {
    num: i64,
    blink: i32,
}

impl CacheEntry {
    fn new(num: i64, blink: i32) -> CacheEntry {
        Self { num, blink }
    }
}

#[derive(Clone, Debug)]
pub struct Stones(Vec<i64>);

impl FromStr for Stones {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Stones(s.split(" ").filter_map(|x| i64::from_str(x.trim()).ok()).collect()))
    }
}

impl Stones {
    pub fn count(&self, blinks: i32) -> usize {
        let mut cache: HashMap<CacheEntry, usize> = HashMap::with_capacity(150000);
        self.0.iter().map(|&x| Self::blink(x, blinks, &mut cache)).sum()
    }

    fn blink(num: i64, blinks: i32, cache: &mut HashMap<CacheEntry, usize>) -> usize {
        if blinks == 0 {
            return 1;
        }

        let entry = CacheEntry::new(num, blinks);
        if cache.contains_key(&entry) {
            return cache[&entry];
        }

        if num == 0 {
            let result = Self::blink(1, blinks - 1, cache);
            cache.insert(entry, result);
            return result;
        }

        let digits = digits(num);
        if digits % 2 == 0 {
            let (first, second) = Self::split_num(num, digits);
            let first_result = Self::blink(first, blinks - 1, cache);
            cache.insert(CacheEntry::new(first, blinks - 1), first_result);

            let second_result = Self::blink(second, blinks - 1, cache);
            cache.insert(CacheEntry::new(second, blinks - 1), second_result);
            return first_result + second_result;
        }

        let result = Self::blink(num * 2024, blinks - 1, cache);
        cache.insert(entry, result);
        result
    }

    #[inline]
    fn split_num(num: i64, digits: u32) -> (i64, i64) {
        let div = 10u32.pow(digits - (digits / 2)) as i64;
        (num / div, num % div)
    }
}
