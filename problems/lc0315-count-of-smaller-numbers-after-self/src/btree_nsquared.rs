pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut out = vec![0; nums.len()];
        let mut stats = BTreeMap::<i32, i32>::new();

        for (idx, value) in nums.into_iter().enumerate().rev() {
            *stats.entry(value).or_default() += 1;
            out[idx] = stats.range(..value).map(|(_, v)| *v).sum();
        }

        out
    }
}
