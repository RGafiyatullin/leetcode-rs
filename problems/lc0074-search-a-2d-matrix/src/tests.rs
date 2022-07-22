use crate::Solution;

const CASES: &[(&[&[i32]], i32, bool)] = &[
    (
        &[&[1, 3, 5, 7], &[10, 11, 16, 20], &[23, 30, 34, 60]],
        13,
        false,
    ),
    (
        &[&[1, 3, 5, 7], &[10, 11, 16, 20], &[23, 30, 34, 60]],
        3,
        true,
    ),
];

#[test]
fn all_cases() {
    for &(m, target, expected) in CASES {
        assert_eq!(
            Solution::search_matrix(m.into_iter().map(|r| r.to_vec()).collect(), target),
            expected
        );
    }
}
