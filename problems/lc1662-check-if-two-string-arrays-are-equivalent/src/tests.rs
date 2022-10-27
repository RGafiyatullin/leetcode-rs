use crate::solution::Solution;


const CASES: &[
    (&[&str], &[&str], bool)
] = &[
    (
        &["ab", "c"], &["a", "bc"], true
    ),
    (
        &["a", "cb"], &["ab", "c"], false,
    ),
    (
        &["abc", "d", "defg"], &["abcddefg"], true,
    )
];

#[test]
fn run_all_cases() {
    for &(left, right, exp) in CASES {
        assert_eq!(Solution::array_strings_are_equal(left.into_iter().map(|s| s.to_string()).collect(), right.into_iter().map(|s| s.to_string()).collect()), exp)
    }
}
