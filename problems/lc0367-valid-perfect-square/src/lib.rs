pub struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num = num as i64;
        assert!(num >= 1);

        use std::cmp::Ordering::*;

        let (mut lo, mut hi) = (1, num);

        loop {
            let mid = (lo + hi) / 2;

            match (lo == hi, (mid * mid).cmp(&num)) {
                (_, Equal) => break true,
                (true, _) => break false,
                (false, Less) => lo = mid + 1,
                (false, Greater) => hi = mid,
            }
        }
    }
}

#[test]
fn run_all_cases() {
    assert!(Solution::is_perfect_square(1));
    assert!(!Solution::is_perfect_square(2));

    assert!(Solution::is_perfect_square(16));
    assert!(!Solution::is_perfect_square(14));

    assert!(Solution::is_perfect_square(1024));
    assert!(Solution::is_perfect_square(1024 * 1024));
    assert!(Solution::is_perfect_square(1024 * 1024 * 1024));

    assert!(!Solution::is_perfect_square(1024 + 1));
    assert!(!Solution::is_perfect_square(1024 * 1024 + 1));
    assert!(!Solution::is_perfect_square(1024 * 1024 * 1024 + 1));
}
