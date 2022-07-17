use crate::Solution;

const CASES: &[(i32, i32, i32)] = &[(3, 0, 1), (3, 1, 2), (1000, 1000, 663677020)];

#[test]
fn run_all_cases() {
    for &(n, k, expected) in CASES {
        assert_eq!(Solution::k_inverse_pairs(n, k), expected);
    }
}
