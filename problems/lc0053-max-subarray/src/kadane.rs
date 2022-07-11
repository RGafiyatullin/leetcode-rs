pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);

        let mut max = nums[0];
        let mut current = nums[0];

        for i in 1..nums.len() {
            current = std::cmp::max(0, current);
            
            current += nums[i];

            max = std::cmp::max(current, max);
        }

        max
    }
}

