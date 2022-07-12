use crate::Solution;

const CASES: &[(&[i32], bool)] = &[(&[1, 1, 2, 2, 2], true), (&[3, 3, 3, 3, 4], false)];

#[test]
fn run_all_cases() {
    for case in CASES {
        assert_eq!(Solution::makesquare(case.0.to_vec()), case.1);
    }
}
