use crate::Solution;

const CASES: &[(&[i32], i32)] = &[
    (&[1], 1),
    (&[0, 1], 2),
    (&[10, 13, 12, 14, 15], 2),
    (&[2, 3, 1, 1, 4], 3),
    (&[5, 1, 3, 4, 2], 3),
];

#[test]
fn run_all_cases() {
    for (input, expected) in CASES {
        assert_eq!(Solution::odd_even_jumps(input.to_vec()), *expected);
    }
}

#[test]
fn run_specific_case() {
    let (input, expected) = &CASES[1];
    assert_eq!(Solution::odd_even_jumps(input.to_vec()), *expected);
}
