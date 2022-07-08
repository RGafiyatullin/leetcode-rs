use crate::Solution;

const CASES: &[(&[&[i32]], i32)] = &[
    (
        &[
            &[1, 1, 1, 1, 1, 1, 1, 0],
            &[1, 0, 0, 0, 0, 1, 1, 0],
            &[1, 0, 1, 0, 1, 1, 1, 0],
            &[1, 0, 0, 0, 0, 1, 0, 1],
            &[1, 1, 1, 1, 1, 1, 1, 0],
        ],
        2,
    ),
    (&[&[0, 0, 1, 0, 0], &[0, 1, 0, 1, 0], &[0, 1, 1, 1, 0]], 1),
    (
        &[
            &[1, 1, 1, 1, 1, 1, 1],
            &[1, 0, 0, 0, 0, 0, 1],
            &[1, 0, 1, 1, 1, 0, 1],
            &[1, 0, 1, 0, 1, 0, 1],
            &[1, 0, 1, 1, 1, 0, 1],
            &[1, 0, 0, 0, 0, 0, 1],
            &[1, 1, 1, 1, 1, 1, 1],
        ],
        2,
    ),
];

fn run_one_case(case: (&[&[i32]], i32)) {
    let (grid, expected) = case;
    let actual = Solution::closed_island(grid.iter().map(|s| s.to_vec()).collect());
    assert_eq!(actual, expected);
}

#[test]
fn run_all_cases() {
    for &case in CASES {
        run_one_case(case);
    }
}
