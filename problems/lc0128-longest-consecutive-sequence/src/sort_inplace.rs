pub struct Solution;

impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        let mut longest_len = 0;

        let mut current_len: usize = 0;
        let mut last_n: Option<i32> = None;

        nums.sort();

        for n in nums {
            current_len = 
                if last_n == Some(n - 1) {
                    current_len + 1
                } else if last_n == Some(n) {
                    current_len
                } else {
                    1
                };
            longest_len = std::cmp::max(longest_len, current_len);
            last_n = Some(n);
        }

        longest_len as i32
    }
}
