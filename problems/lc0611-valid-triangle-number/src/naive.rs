pub struct Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 1000);

        nums.sort_unstable();

        let mut count = 0;
        for (idx_1, first) in nums.iter().copied().enumerate() {
            for (idx_2, second) in nums[idx_1 + 1..].iter().copied().enumerate() {
                for third in nums[idx_1 + 1 + idx_2 + 1..].iter().copied() {
                    if third < second + first {
                        eprintln!("* {:?} {:?} {:?}", first, second, third);
                        count += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        count
    }
}
