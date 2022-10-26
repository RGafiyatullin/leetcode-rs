use crate::Solution;

const CASES: &[(&[&[i32]], &[&[i32]], i32)] = &[
    (
        &[&[1, 1, 1, 0, 0], &[0, 1, 1, 1, 1], &[0, 0, 0, 0, 0], &[1, 0, 0, 0, 0], &[1, 1, 0, 1, 1]],
        &[&[1, 1, 1, 0, 0], &[0, 0, 1, 1, 1], &[0, 1, 0, 0, 0], &[1, 0, 1, 1, 0], &[0, 1, 0, 1, 0]],
        3,
    ),
    (
        &[&[1, 0, 1, 0, 1], &[1, 1, 1, 1, 1], &[0, 0, 0, 0, 0], &[1, 1, 1, 1, 1], &[1, 0, 1, 0, 1]],
        &[&[0, 0, 0, 0, 0], &[1, 1, 1, 1, 1], &[0, 1, 0, 1, 0], &[0, 1, 0, 1, 0], &[1, 0, 0, 0, 1]],
        2,
    ),
];

fn run_one_case(case: (&[&[i32]], &[&[i32]], i32)) {
    let (g1, g2, expected) = case;
    let g1 = g1.iter().map(|row| row.to_vec()).collect();
    let g2 = g2.iter().map(|row| row.to_vec()).collect();
    let actual = Solution::count_sub_islands(g1, g2);

    assert_eq!(actual, expected);
}

#[test]
fn run_all_cases() {
    for case in CASES {
        run_one_case(*case);
    }
}
