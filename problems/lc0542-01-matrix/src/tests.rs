use crate::Solution;

#[test]
fn run_all_cases() {
    for case in CASES {
        run_one_case(*case);
    }
}

fn run_one_case((input, expected): (&[&[i32]], &[&[i32]])) {
    let actual = Solution::update_matrix(input.into_iter().map(|r| r.to_vec()).collect());
    assert_eq!(actual, expected);
}

const CASES: &[(&[&[i32]], &[&[i32]])] = &[
    (&[&[0, 0, 0], &[0, 1, 0], &[0, 0, 0]], &[&[0, 0, 0], &[0, 1, 0], &[0, 0, 0]]),
    (&[&[0, 0, 0], &[0, 1, 0], &[1, 1, 1]], &[&[0, 0, 0], &[0, 1, 0], &[1, 2, 1]]),
];
