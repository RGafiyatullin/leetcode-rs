use super::*;

const CASES: &[(i32, bool)] = &[(1, false), (2, true), (3, false)];

#[test]
fn run_all_cases() {
    for &(n, outcome) in CASES {
        eprintln!("RUNNING {:?}", n);
        assert_eq!(Solution::divisor_game(n), outcome, "CASE: {:?}", n);
    }
}
