use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Patterns {
    designs: Vec<String>,
    patterns: HashSet<String>,
    max_pattern: usize,
}

impl FromStr for Patterns {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split("\n\n").collect();
        let patterns: HashSet<_> = splits[0].split(", ").map(String::from).collect();
        let designs = splits[1].split("\n").map(String::from).collect();
        let max_pattern = patterns.iter().map(|x| x.len()).max().unwrap_or(0);
        Ok(Self { designs, patterns, max_pattern })
    }
}

impl Patterns {
    pub fn possible(&self) -> usize {
        let mut cache = HashMap::new();
        self.designs.iter().filter(|&x| self.possible_count(x, &mut cache) > 0).count()
    }

    pub fn all_ways(&self) -> usize {
        let mut cache = HashMap::new();
        self.designs.iter().map(|x| self.possible_count(x, &mut cache)).sum()
    }

    fn possible_count(&self, design: &str, cache: &mut HashMap<String, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }

        let mut count = 0;
        for idx in 0..=min(design.len(), self.max_pattern) {
            if self.patterns.contains(&design[..idx]) {
                if cache.contains_key(&design[idx..]) {
                    count += cache[&design[idx..]];
                } else {
                    count += self.possible_count(&design[idx..], cache);
                }
            }
        }
        cache.insert(String::from(design), count);
        count
    }
}
