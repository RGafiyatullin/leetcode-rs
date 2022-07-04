const CASES: &[(&[i32], i32)] = &[
    (&[1, 7, 4, 9, 2, 5], 6),
    (&[1, 17, 5, 10, 13, 15, 10, 5, 16, 8], 7),
    (&[1, 2, 3, 4, 5, 6, 7, 8, 9], 2),
];

#[test]
fn test_local_extrema() {
    for (input, expected) in CASES {
        assert_eq!(
            crate::local_extrema::Solution::wiggle_max_length(input.to_vec()),
            *expected
        );
    }
}
