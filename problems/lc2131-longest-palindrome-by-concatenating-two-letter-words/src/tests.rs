const CASES: &[(&[&str], i32)] = &[
    (&["lc", "cl", "gg"], 6),
    (&["ab", "ty", "yt", "lc", "cl", "ab"], 8),
    (&["cc", "ll", "xx"], 2),
];

#[test]
fn all_cases() {
    for &(input, expected) in CASES {
        eprintln!("input: {:?}", input);
        assert_eq!(
            crate::solution::Solution::longest_palindrome(
                input.into_iter().copied().map(ToOwned::to_owned).collect()
            ),
            expected
        );
    }
}
