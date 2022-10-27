use crate::solution::Solution;

const CASES: &[(&[i32], i32, bool)] = &[
    (&[23, 2, 4, 6, 7], 6, true),
    (&[23, 2, 6, 4, 7], 6, true),
    (&[23, 2, 6, 4, 7], 13, false),
    (&[23, 2, 4, 6, 6], 7, true),
    (&[1, 2, 3], 5, true),
];

#[test]
fn run_all_cases() {
    for &(nums, k, exp) in CASES {
        assert_eq!(Solution::check_subarray_sum(nums.to_owned(), k), exp);
    }
}

#[test]
fn run_specific_case() {
    let (nums, k, exp) = CASES.last().copied().unwrap();
    assert_eq!(Solution::check_subarray_sum(nums.to_owned(), k), exp)
}
