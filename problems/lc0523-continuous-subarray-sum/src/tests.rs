use crate::solution::Solution;

const CASES: &[(&[i32], i32, bool)] =
    &[(&[23, 2, 4, 6, 7], 6, true), (&[23, 2, 6, 4, 7], 6, true), (&[23, 2, 6, 4, 7], 13, false)];

#[test]
fn run_all_cases() {
    for &(nums, k, exp) in CASES {
        assert_eq!(Solution::check_subarray_sum(nums.to_owned(), k), exp);
    }
}
