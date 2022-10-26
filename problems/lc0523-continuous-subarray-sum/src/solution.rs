pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut rems = HashMap::<i32, usize>::new();
        let mut sum = 0;

        for (idx, value) in nums.into_iter().enumerate() {
            sum += value;

            let rem = sum % k;
            
            if rem == 0 && idx > 0 {
                return true
            } 

            if let Some(former_idx) = rems.get(&rem).copied() {
                assert!(idx > former_idx);
                if idx - former_idx >= 2 {
                    return true
                }
            } else {
                rems.insert(rem, idx);
            }
        }
        false
    }
}
