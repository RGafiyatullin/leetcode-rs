pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 5000);

        nums.into_iter()
            .enumerate()
            .find(|(_, v)| *v == target)
            .map(|(i, _)| i as i32)
            .unwrap_or(-1)
    }
}
