use crate::Solution;

const CASES: &[(&[i32], i32)] = &[
    (&[10, 15, 20], 15),
    (&[1, 100, 1, 1, 1, 100, 1, 1, 100, 1], 6),
];

fn run_one_case(case: (&[i32], i32)) {
    let (cost, expected) = case;
    let actual = Solution::min_cost_climbing_stairs(cost.to_vec());
    assert_eq!(actual, expected);
}

#[test]
fn run_all_cases() {
    for case in CASES {
        run_one_case(*case);
    }
}
