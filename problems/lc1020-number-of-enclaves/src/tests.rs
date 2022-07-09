use crate::Solution;

const CASES: &[
    (&[&[i32]], i32)
] = &[
    (&[&[0,0,0,0],&[1,0,1,0],&[0,1,1,0],&[0,0,0,0]], 3),
    (&[&[0,1,1,0],&[0,0,1,0],&[0,0,1,0],&[0,0,0,0]], 0),
];

fn run_one_case(case: (&[&[i32]], i32)) {
    let (grid, expected) = case;
    let actual = Solution::num_enclaves(grid.iter().map(|r| r.to_vec()).collect());
    assert_eq!(actual, expected, "grid: {:?}", grid);
}

#[test]
fn run_all_cases() {
    for case in CASES {
        run_one_case(*case);
    }
}
