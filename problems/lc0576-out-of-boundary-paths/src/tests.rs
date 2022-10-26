use crate::Solution;

const CASES: &[(i32, i32, i32, i32, i32, i32)] =
    &[(2, 2, 2, 0, 0, 6), (1, 3, 3, 0, 1, 12), (8, 50, 23, 5, 26, 914783380)];

#[test]
fn run_all_cases() {
    for &(m, n, max, row, col, expected) in CASES {
        assert_eq!(Solution::find_paths(m, n, max, row, col), expected);
    }
}
