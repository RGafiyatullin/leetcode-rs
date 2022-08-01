pub struct Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        nums.iter()
            .copied()
            .enumerate()
            .map(|(idx_lo, val_lo)| (val_lo, &nums[idx_lo..]))
            .flat_map(|(val_lo, slice)| {
                slice
                    .iter()
                    .copied()
                    .enumerate()
                    .rev()
                    .find_map(|(ramp_len, val_hi)| Some(ramp_len).filter(|_| val_hi >= val_lo))
            })
            .max()
            .unwrap_or(0) as i32
    }
}
