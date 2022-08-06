use crate::*;

const CASES: &[(i32, i32, i32, i32)] = &[(1000, 15, 60, 5), (4, 15, 15, 2), (4, 15, 30, 2)];

#[test]
fn run_all_cases() {
    for &(buckets, mins_to_die, mins_to_test, exp) in CASES {
        assert_eq!(Solution::poor_pigs(buckets, mins_to_die, mins_to_test), exp);
    }
}
