use crate::Solution;

const CASES: &[(i32, bool)] = &[
    (5, true),
    (3, false),
    ((1 << 30) + 4, true),
    ((1 << 30) + 3, false),
    (32752 * 32752 + 32753 + 32753, true),
    (32752 * 32752 * 2 + 1, false),
];

#[test]
fn run_all_cases() {
    for &(c, exp) in CASES {
        assert_eq!(Solution::judge_square_sum(c), exp);
    }
}
