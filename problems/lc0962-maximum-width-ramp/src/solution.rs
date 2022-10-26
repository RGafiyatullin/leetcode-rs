pub struct Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mins: Vec<_> = nums
            .iter()
            .copied()
            .scan(None, |min_opt, val| {
                let min = if let Some(min) = *min_opt { std::cmp::min(val, min) } else { val };
                *min_opt = Some(min);
                *min_opt
            })
            .collect();

        assert_eq!(mins.len(), nums.len());

        let mut best = 0;

        for (idx, val) in nums.iter().copied().enumerate().rev() {
            let candidate_idxs = (0..idx.saturating_sub(best)).rev();

            for candidate_idx in candidate_idxs {
                if mins[candidate_idx] > val {
                    break
                }

                if nums[candidate_idx] <= val {
                    best = std::cmp::max(best, idx - candidate_idx);
                }
            }
        }

        best as i32
    }
}
