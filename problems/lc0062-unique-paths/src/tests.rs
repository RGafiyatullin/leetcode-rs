use crate::Solution;

const CASES: &[(i32, i32, i32)] = &[(3, 7, 28), (3, 2, 3)];

#[test]
fn run_all_cases() {
    for &(m, n, exp) in CASES {
        let act = Solution::unique_paths(m, n);
        assert_eq!(act, exp, "{:?}*{:?} grid", m, n);
    }
}
