use crate::Solution;

const CASES: &[(&[i32], i32)] = &[(&[1, 2, 3, 1], 4), (&[2, 7, 9, 3, 1], 12), (&[2, 3, 2], 4)];

fn run_one_case(case: (&[i32], i32)) {
    assert_eq!(Solution::rob(case.0.to_vec()), case.1);
}

#[test]
fn run_all_cases() {
    for case in CASES {
        run_one_case(*case);
    }
}


#[test]
fn run_specific_case() {
    run_one_case(CASES[2]);
}