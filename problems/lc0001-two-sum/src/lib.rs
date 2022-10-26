pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut value_to_index = HashMap::<i32, usize>::new();

        for (idx, value) in nums.into_iter().enumerate() {
            let complement = target - value;

            if let Some(&complement_idx) = value_to_index.get(&complement) {
                return vec![complement_idx as i32, idx as i32]
            }

            value_to_index.insert(value, idx);
        }

        panic!("we agreed on having exactly one solution!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn case_02() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn case_03() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
