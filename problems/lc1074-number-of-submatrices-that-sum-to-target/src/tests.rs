use crate::Solution;

const CASES: &[(&[&[i32]], i32, i32)] = &[
    (&[&[0, 1, 0], &[1, 1, 1], &[0, 1, 0]], 0, 4),
    (&[&[1, -1], &[-1, 1]], 0, 5),
    (&[&[904]], 0, 0),
];

#[test]
fn run_all_cases() {
    for &(matrix, target, expected) in CASES {
        assert_eq!(
            Solution::num_submatrix_sum_target(
                matrix.into_iter().map(|row| row.to_vec()).collect(),
                target
            ),
            expected
        );
    }
}
