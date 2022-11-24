const CASES: &[(&[&[char]], &str, bool)] = &[
    (&[&['A', 'B', 'C', 'E'], &['S', 'F', 'C', 'S'], &['A', 'D', 'E', 'E']], "ABCCED", true),
    (&[&['A', 'B', 'C', 'E'], &['S', 'F', 'C', 'S'], &['A', 'D', 'E', 'E']], "SEE", true),
    (&[&['A', 'B', 'C', 'E'], &['S', 'F', 'C', 'S'], &['A', 'D', 'E', 'E']], "ABCB", false),
    (&[&['a']], "a", true),
];

#[test]
fn run_all_cases() {
    for (idx, &(board, word, expected)) in CASES.into_iter().enumerate() {
        let board = board.into_iter().map(|r| r.to_vec()).collect();
        assert_eq!(
            crate::solution::Solution::exist(board, word.to_owned()),
            expected,
            "idx: {}",
            idx
        );
    }
}
