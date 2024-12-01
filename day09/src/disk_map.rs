use std::str::FromStr;

const EMPTY: usize = usize::MAX;

#[derive(Clone, Debug)]
pub struct DiskMap {
    map: Vec<u8>,
    expanded: Vec<usize>,
}

impl FromStr for DiskMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<_> = s.chars().filter_map(|x| x.to_digit(10).map(|x| x as u8)).collect();
        let mut expanded: Vec<usize> = Vec::with_capacity(100000);
        for (idx, &num) in map.iter().enumerate() {
            if idx % 2 == 0 {
                expanded.extend(vec![idx / 2; num as usize]);
            } else {
                expanded.extend(vec![EMPTY; num as usize]);
            }
        }
        Ok(Self { map, expanded })
    }
}

impl DiskMap {
    pub fn bit_move_checksum(&self) -> usize {
        let mut compacted = self.expanded.clone();
        let mut space_idx = Self::next_space_idx(0, &compacted);
        for file_idx in (0..compacted.len()).rev() {
            if file_idx < space_idx {
                break;
            }
            if compacted[file_idx] != EMPTY {
                compacted.swap(space_idx, file_idx);
                space_idx = Self::next_space_idx(space_idx, &compacted);
            }
        }
        Self::checksum(&compacted)
    }

    pub fn file_move_checksum(&self) -> usize {
        let mut compacted = self.expanded.clone();
        for file_idx in (0..self.map.len()).rev().step_by(2) {
            let file_size = self.map[file_idx] as usize;
            let file_start = self.start_chunk_idx(file_idx);
            let next_empty = Self::next_empty_chunk(file_size, &compacted);
            if next_empty < file_start {
                Self::move_file(file_start, next_empty, &mut compacted);
            }
        }
        Self::checksum(&compacted)
    }

    fn checksum(compacted: &[usize]) -> usize {
        compacted
            .iter()
            .enumerate()
            .filter(|(_, &num)| num != EMPTY)
            .map(|(idx, &num)| idx * num)
            .sum()
    }

    fn next_space_idx(idx: usize, nums: &[usize]) -> usize {
        let mut result = idx;
        while result < nums.len() && nums[result] != EMPTY {
            result += 1;
        }
        result
    }

    fn next_empty_chunk(size: usize, nums: &[usize]) -> usize {
        let mut cur_size = 0;
        for (idx, &num) in nums.iter().enumerate() {
            if num == EMPTY {
                cur_size += 1;
            } else {
                cur_size = 0;
            }

            if cur_size >= size {
                return idx - cur_size + 1;
            }
        }
        usize::MAX
    }

    fn start_chunk_idx(&self, idx: usize) -> usize {
        if idx == 0 { 0 } else { self.map[0..idx].iter().map(|&x| x as usize).sum() }
    }

    fn move_file(file_idx: usize, space_idx: usize, nums: &mut [usize]) {
        let file_id = nums[file_idx];
        let mut cur_file_idx = file_idx;
        let mut cur_space_idx = space_idx;
        while cur_file_idx < nums.len() && nums[cur_file_idx] == file_id {
            nums.swap(cur_file_idx, cur_space_idx);
            cur_file_idx += 1;
            cur_space_idx += 1;
        }
    }
}
