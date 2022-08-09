use crate::Solution;

const CASES: &[(&[i32], i32)] = &[
    (&[10, 9, 2, 5, 3, 7, 101, 18], 4),
    (&[0, 1, 0, 3, 2, 3], 4),
    (&[7, 7, 7, 7, 7, 7, 7], 1),
];

#[test]
fn run_all_cases() {
    for &(input, exp) in CASES {
        assert_eq!(Solution::length_of_lis(input.to_vec()), exp);
    }
}
