const CASES: &[(i32, bool)] = &[(6, true), (1, true), (14, false)];

#[test]
fn run_all_cases() {
    for &(n, exp) in CASES {
        assert_eq!(crate::solution::Solution::is_ugly(n), exp);
    }
}
