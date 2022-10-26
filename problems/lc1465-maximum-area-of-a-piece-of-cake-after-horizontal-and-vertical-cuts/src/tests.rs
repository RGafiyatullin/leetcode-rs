use super::solution::Solution;

const CASES: &[(i32, i32, &[i32], &[i32], i32)] =
    &[(5, 4, &[3, 1], &[1], 6), (5, 5, &[3], &[3], 9)];

#[test]
fn test_all_cases() {
    for (h, w, hc, vc, expected) in CASES {
        assert_eq!(Solution::max_area(*h, *w, hc.to_vec(), vc.to_vec()), *expected);
    }
}
