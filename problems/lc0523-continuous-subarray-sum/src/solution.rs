pub struct Solution;

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut rems: HashMap<i32, usize> = [(0, 0)].into();
        let mut sum = 0;

        for (idx, value) in nums.into_iter().enumerate() {
            let idx = idx + 1;

            sum += value;
            let rem = sum % k;

            match rems.entry(rem) {
                Entry::Occupied(occupied) =>
                    if idx - *occupied.get() > 1 {
                        return true
                    },
                Entry::Vacant(vacant) => {
                    vacant.insert(idx);
                },
            }
        }
        false
    }
}
