use crate::solution::Solution;

const CASES: &[(&[&[i32]], &[&[i32]], i32)] = &[
    
    (&[&[1]], &[&[1]], 1),
    (&[&[0]], &[&[0]], 0),
    (
        &[&[0, 0, 0, 0, 1], &[0, 0, 0, 0, 0], &[0, 0, 0, 0, 0], &[0, 0, 0, 0, 0], &[0, 0, 0, 0, 0]],
        &[&[0, 0, 0, 0, 0], &[0, 0, 0, 0, 0], &[0, 0, 0, 0, 0], &[0, 0, 0, 0, 0], &[1, 0, 0, 0, 0]],
        1,
    ),

    (&[&[1, 1, 0], &[0, 1, 0], &[0, 1, 0]], &[&[0, 0, 0], &[0, 1, 1], &[0, 0, 1]], 3),
];

fn run_one_case((left, right, exp): (&[&[i32]], &[&[i32]], i32)) {
    let left = left.into_iter().map(|s| s.to_vec()).collect();
    let right = right.into_iter().map(|s| s.to_vec()).collect();

    eprintln!("====");
    eprintln!("Left:  {:?}", left);
    eprintln!("Right: {:?}", right);
    assert_eq!(Solution::largest_overlap(left, right), exp);
}

#[test]
fn run_all_cases() {
    for &case in CASES {
        run_one_case(case);
    }
}

#[test]
fn run_last_case() {
    run_one_case(CASES.last().copied().unwrap())
}
