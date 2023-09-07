pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let max = nums.len();
        let set = nums.iter().copied().collect::<HashSet<_>>();

        nums.into_iter().map(|n| seq_len(n, max, &set)).max().unwrap_or(0) as i32
    }
}

fn seq_len(n: i32, max: usize, set: &HashSet<i32>) -> usize {
    let prev = n - 1;
    if set.contains(&prev) {
        return 0
    }

    for i in 1..=max {
        let m = n + i as i32;
        if !set.contains(&m) {
            return i
        }
    }
    panic!("something's wrong with the set")
}
