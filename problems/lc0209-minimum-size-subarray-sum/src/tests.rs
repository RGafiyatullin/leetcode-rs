use crate::Solution;

const CASES: &[(i32, &[i32], i32)] =
    &[(7, &[2, 3, 1, 2, 4, 3], 2), (4, &[1, 4, 4], 1), (11, &[1, 1, 1, 1, 1, 1, 1, 1], 0)];

#[test]
fn run_all_cases() {
    for &(target, nums, exp) in CASES {
        eprintln!("*** {:?} | {:?} -> {:?}", target, nums, exp);
        assert_eq!(Solution::min_sub_array_len(target, nums.to_vec()), exp);
    }
}
