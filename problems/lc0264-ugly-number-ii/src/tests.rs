const CASES: &[(i32, i32)] = &[
    (1, 1),
    (2, 2),
    (3, 3),
    (4, 4),
    (5, 5),
    (6, 6),
    (7, 8),
    (8, 9),
    (9, 10),
    (10, 12),
    (11, 15),
    (12, 16),
    (13, 18),
    (14, 20),
    // (1352, 402653184),
];

#[test]
fn run_all_cases() {
    for &(n, exp) in CASES {
        assert_eq!(crate::solution::Solution::nth_ugly_number(n), exp);
    }
}

#[test]
fn do_1352_expect_402653184() {
    assert_eq!(crate::solution::Solution::nth_ugly_number(1352), 402653184);
}
