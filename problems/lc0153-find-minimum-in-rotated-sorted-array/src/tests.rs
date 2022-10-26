use crate::Solution;

const CASES: &[(&[i32], i32)] =
    &[(&[3, 4, 5, 1, 2], 1), (&[4, 5, 6, 7, 0, 1, 2], 0), (&[11, 13, 15, 17], 11)];

#[test]
fn run_all_cases() {
    for &(nums, exp) in CASES {
        assert_eq!(Solution::find_min(nums.to_vec()), exp);
    }
}
