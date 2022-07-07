use crate::Solution;

mod cases;
use cases::*;

#[test]
fn test_leetcode_cases() {
    for &case in LEETCODE_CASES {
        run_one_case(case)
    }
}

#[test]
fn test_my_cases() {
    for &case in MY_CASES {
        run_one_case(case)
    }
}

#[test]
fn specific_case() {
    run_one_case(MY_CASES[6])
}

fn run_one_case(case: (&[&[char]], i32)) {
    let (grid, expected) = case;
    let actual = Solution::num_islands(grid.iter().map(|s| s.to_vec()).collect());
    assert_eq!(actual, expected);
}

