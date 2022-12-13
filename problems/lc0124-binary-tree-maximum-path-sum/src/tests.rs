const CASES: &[(&[Option<i32>], i32)] = &[
    (&[Some(1), Some(2), Some(3)], 6),
    (&[Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)], 42),
];

#[test]
fn run_all_cases() {
    for (case_idx, &(input, expected)) in CASES.into_iter().enumerate() {
        let tree = utils::tree::tree_from_bft(input);
        assert_eq!(crate::solution::Solution::max_path_sum(tree), expected, "CASE[{}]", case_idx);
    }
}
