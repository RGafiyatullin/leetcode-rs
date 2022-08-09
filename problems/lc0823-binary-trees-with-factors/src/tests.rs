use crate::Solution;

const CASES: &[(&[i32], i32)] = &[(&[2, 4], 3), (&[2, 4, 5, 10], 7)];

#[test]
fn run_all_cases() {
    for &(input, exp) in CASES {
        eprintln!("{:?} => {:?}", input, exp);
        assert_eq!(Solution::num_factored_binary_trees(input.to_vec()), exp);
    }
}
