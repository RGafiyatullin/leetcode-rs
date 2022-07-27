use crate::Solution;

const CASES: &[(&[Option<i32>], &[Option<i32>])] = &[
    (
        &[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)],
        &[
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ],
    ),
    (&[], &[]),
    (&[Some(0)], &[Some(0)]),
];

#[test]
fn test_all_cases() {
    for &(input_bft, expected_bft) in CASES {
        let mut tree = utils::tree::tree_from_bft(input_bft);
        Solution::flatten(&mut tree);
        let output_bft = utils::tree::tree_to_bft(tree);
        assert_eq!(output_bft, expected_bft);
    }
}
