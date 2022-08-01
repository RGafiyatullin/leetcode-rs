use crate::Solution;

const CASES: &[(&[i32], i32)] = &[
    (&[6, 0, 8, 2, 1, 5], 4),
    (&[9, 8, 1, 0, 1, 9, 4, 0, 4, 1], 7),
];

#[test]
fn run_all_cases() {
    for &(input, expected) in CASES {
        assert_eq!(Solution::max_width_ramp(input.to_vec()), expected);
    }
}

#[test]
fn run_long_test_01() {
    assert_eq!(Solution::max_width_ramp((0..50000).rev().collect()), 0);
}
#[test]
fn run_long_test_02() {
    assert_eq!(Solution::max_width_ramp((0..50000).collect()), 49999);
}
