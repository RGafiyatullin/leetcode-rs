use crate::Solution;

const CASES: &[(&[i32], i32)] = &[
    (&[1,2,1], 3),
    (&[0,1,2,2], 3),
    (&[1,2,3,2,2], 4),
];

fn run_one_case(case: (&[i32], i32)) {
    let (input, expected) = case;
    let actual = Solution::total_fruit(input.to_vec());
    assert_eq!(actual, expected);
}

#[test]
fn many_trees_all_different() {
    run_one_case(((0..100000).collect::<Vec<_>>().as_slice(), 2))
}
#[test]
fn many_trees_two_alternating_types() {
    run_one_case(((0..100000).map(|idx| idx % 2).collect::<Vec<_>>().as_slice(), 100000));
}

#[test]
fn run_all_cases() {
    for case in CASES.iter().cloned() {
        run_one_case(case);
    }
}
