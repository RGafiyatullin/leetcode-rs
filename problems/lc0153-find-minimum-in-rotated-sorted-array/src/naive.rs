pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 5000);

        nums.into_iter().min().expect("nums.len() >= 1")
    }
}
