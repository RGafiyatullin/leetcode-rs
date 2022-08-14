use crate::Solution;

const CASES: &[(&[i32], &[i32], i32)] = &[
    (&[55, 30, 5, 4, 2], &[100, 20, 10, 10, 5], 2),
    (&[2, 2, 2], &[10, 10, 1], 1),
    (&[30, 29, 19, 5], &[25, 25, 25, 25, 25], 2),
];

#[test]
fn run_all_cases() {
    for &(nums1, nums2, exp) in CASES {
        eprintln!("=== {:?} | {:?} => {:?}", nums1, nums2, exp);
        assert_eq!(Solution::max_distance(nums1.to_vec(), nums2.to_vec()), exp);
    }
}
