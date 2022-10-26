use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let tree = nums.into_iter().collect::<BTreeSet<_>>();

        let mut longest_len = 0;

        let mut current_len: usize = 0;
        let mut expected_n: Option<i32> = None;

        for n in tree.range(..).copied() {
            current_len = if expected_n == Some(n) { current_len + 1 } else { 1 };
            longest_len = std::cmp::max(longest_len, current_len);
            expected_n = Some(n + 1);
        }

        longest_len as i32
    }
}
