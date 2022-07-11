pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let min_step = 2;
        let max_step = 3;

        let mut memo = vec![0; nums.len()];

        for (idx, value) in nums.into_iter().enumerate() {
            let candidates = (idx.saturating_sub(max_step)..=idx.saturating_sub(min_step))
                .filter(|from_idx| idx - *from_idx >= min_step)
                .inspect(|from_idx| eprintln!("candidate {} -> {} [{}]", from_idx, idx, memo[*from_idx]))
                .map(|from_idx| memo[from_idx]);

            memo[idx] = candidates.max().unwrap_or(0) + value;
            eprintln!("memo[{}] <- {}", idx, memo[idx]);
        }

        memo.into_iter().max().unwrap_or(0)
    }
}
