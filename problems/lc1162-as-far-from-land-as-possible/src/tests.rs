use crate::Solution;

const CASES: &[(&[&[i32]], i32)] = &[
    (&[&[1, 0, 1], &[0, 0, 0], &[1, 0, 1]], 2),
    (&[&[1, 0, 0], &[0, 0, 0], &[0, 0, 0]], 4),
    (&[&[1,1,1,1,1],&[1,1,1,1,1],&[1,1,1,1,1],&[1,1,1,1,1],&[1,1,1,1,1]], -1),
    (&[&[0,0,0,0,0],&[0,0,0,0,0],&[0,0,0,0,0],&[0,0,0,0,0],&[0,0,0,0,0]], -1),
];

fn run_one_case(case: (&[&[i32]], i32)) {
    let (grid, expected) = case;
    let grid: Vec<Vec<i32>> = grid.iter().map(|row| row.to_vec()).collect();
    let actual = Solution::max_distance(grid);
    assert_eq!(actual, expected);
}

#[test]
fn run_all_cases() {
    for case in CASES.iter() {
        run_one_case(*case);
    }
}
