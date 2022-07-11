pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        let mut max = nums[0];

        for start_idx in 0..nums.len() {
            let mut sum = nums[start_idx];

            max = std::cmp::max(sum, max);

            for end_idx in (start_idx + 1)..nums.len() {
                sum += nums[end_idx];
                max = std::cmp::max(sum, max);
            }
        }

        max
    }
}
