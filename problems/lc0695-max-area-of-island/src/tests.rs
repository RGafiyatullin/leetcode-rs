use crate::Solution;

const CASES: &[(&[&[i32]], i32)] = &[
    (&[&[0,0,1,0,0,0,0,1,0,0,0,0,0],&[0,0,0,0,0,0,0,1,1,1,0,0,0],&[0,1,1,0,1,0,0,0,0,0,0,0,0],&[0,1,0,0,1,1,0,0,1,0,1,0,0],&[0,1,0,0,1,1,0,0,1,1,1,0,0],&[0,0,0,0,0,0,0,0,0,0,1,0,0],&[0,0,0,0,0,0,0,1,1,1,0,0,0],&[0,0,0,0,0,0,0,1,1,0,0,0,0]], 6)

];


fn run_one_case(case: (&[&[i32]], i32)) {
    let (grid, expected) = case;
    let actual = Solution::num_islands(grid.iter().map(|s| s.to_vec()).collect());
    assert_eq!(actual, expected);
}

#[test]
fn run_all_cases() {
    for &case in CASES {
        run_one_case(case);
    }
}
