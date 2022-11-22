use crate::vecdeque_with_memo::Solution;

const CASES: &[(i32, i32)] = &[
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 1),
    (5, 2),
    (6, 3),
    (7, 4),
    (8, 2),
    (9, 1),
    (10, 2),
    (11, 3),
    (12, 3),
    (13, 2),
    (8935, 4),
];

#[test]
fn run_all_cases() {
    for &(input, expected) in CASES {
        assert_eq!(Solution::num_squares(input), expected, "IN: {}", input);
    }
}
