pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let min_step = 2;
        let max_step = 3;

        if nums.len() == 1 {
            return nums[0];
        }

        let mut memo = vec![0; nums.len() - 1];

        std::cmp::max(
            rob(min_step, max_step, &nums[1..], memo.as_mut_slice()),
            rob(
                min_step,
                max_step,
                &nums[0..(nums.len() - 1)],
                memo.as_mut_slice(),
            ),
        )
    }
}

fn rob(min_step: usize, max_step: usize, nums: &[i32], memo: &mut [i32]) -> i32 {
    for (idx, value) in nums.iter().copied().enumerate() {
        let candidates = (idx.saturating_sub(max_step)..=idx.saturating_sub(min_step))
            .filter(|from_idx| idx - *from_idx >= min_step)
            // .inspect(|from_idx| eprintln!("candidate {} -> {} [{}]", from_idx, idx, memo[*from_idx]))
            .map(|from_idx| memo[from_idx]);

        memo[idx] = candidates.max().unwrap_or(0) + value;
        // eprintln!("memo[{}] <- {}", idx, memo[idx]);
    }

    memo.iter().copied().max().unwrap_or(0)
}
