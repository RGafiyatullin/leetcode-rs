use crate::Solution;

const CASES: &[(&[&[i32]], i32, i32, i32, &[&[i32]])] =
    &[(&[&[1, 1, 1], &[1, 1, 0], &[1, 0, 1]], 1, 1, 2, &[&[2, 2, 2], &[2, 2, 0], &[2, 0, 1]])];

#[test]
fn test_all_cases() {
    for &case in CASES {
        run_one_case(case);
    }
}

fn run_one_case(case: (&[&[i32]], i32, i32, i32, &[&[i32]])) {
    let (matrix, sr, sc, new_color, expected) = case;
    let matrix = matrix.iter().copied().map(Vec::from).collect::<Vec<_>>();
    let expected = expected.iter().copied().map(Vec::from).collect::<Vec<_>>();
    let actual = Solution::flood_fill(matrix.to_owned(), sr, sc, new_color);

    assert_eq!(
        actual, expected,
        "matrix: {:?}, sr: {}, sc: {}, new_color: {}, expected: {:?}",
        matrix, sr, sc, new_color, expected
    );
}
