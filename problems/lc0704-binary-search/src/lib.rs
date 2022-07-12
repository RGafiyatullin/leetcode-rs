pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        assert!(nums.len() >= 1);

        let mut start = 0;
        let mut end = nums.len();

        let pos = loop {
            // eprintln!("[{}..{}]", start, end);
            if start >= end {
                break None;
            }
            use std::cmp::Ordering;

            let mid = (start + end) / 2;

            match target.cmp(&nums[mid]) {
                Ordering::Equal => break Some(mid),
                Ordering::Greater => {
                    start = mid + 1;
                }
                Ordering::Less => {
                    end = mid;
                }
            }
        };

        pos.map(|i| i as i32).unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
