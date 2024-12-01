use std::collections::{HashMap, HashSet};
use std::str::FromStr;

/// A map between page numbers and all the page numbers.
#[derive(Clone, Debug)]
pub struct PageOrderMap {
    /// A map between a page number and all the pages after it (i.e., before: after).
    pub after_map: HashMap<i32, HashSet<i32>>,

    /// A map between a page number and all the pages before it (i.e., after: before).
    pub before_map: HashMap<i32, HashSet<i32>>,
}

impl Default for PageOrderMap {
    fn default() -> PageOrderMap {
        Self {
            after_map: HashMap::with_capacity(100),
            before_map: HashMap::with_capacity(100),
        }
    }
}

impl PageOrderMap {
    pub fn add(&mut self, item: &str) {
        // item = before|after
        let splits: Vec<_> = item.split("|").collect();
        let before = i32::from_str(splits[0]).unwrap();
        let after = i32::from_str(splits[1]).unwrap();
        self.after_map.entry(before).or_default().insert(after);
        self.before_map.entry(after).or_default().insert(before);
    }

    pub fn is_valid(&self, before: i32, after: i32) -> bool {
        if self.after_map.contains_key(&before) && self.after_map[&before].contains(&after) {
            return true;
        }
        if self.before_map.contains_key(&after) && self.before_map[&after].contains(&before) {
            return true;
        }
        false
    }
}
