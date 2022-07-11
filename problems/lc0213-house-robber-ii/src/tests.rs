use crate::Solution;

const CASES: &[(&[i32], i32)] = &[(&[2, 3, 2], 3), (&[1, 2, 3, 1], 4), (&[1], 1)];

fn run_one_case(case: (&[i32], i32)) {
    assert_eq!(Solution::rob(case.0.to_vec()), case.1);
}

#[test]
fn run_all_cases() {
    for case in CASES {
        run_one_case(*case);
    }
}
