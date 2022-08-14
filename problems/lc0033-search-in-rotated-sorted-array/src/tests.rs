use crate::Solution;

const CASES: &[(&[i32], i32, i32)] = &[
    (&[4, 5, 6, 7, 0, 1, 2], 0, 4),
    (&[4, 5, 6, 7, 0, 1, 2], 3, -1),
    (&[1], 0, -1),
];

#[test]
fn run_all_cases() {
    for &(nums, target, exp) in CASES {
        eprintln!("{:?} | {:?} => {:?}", nums, target, exp);
        assert_eq!(Solution::search(nums.to_vec(), target), exp);
    }
}
