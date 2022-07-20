use crate::Solution;

const CASES: &[(&str, &[&str], i32)] = &[
    ("abcde", &["a", "bb", "acd", "ace"], 3),
    (
        "dsahjpjauf",
        &["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"],
        2,
    ),
];

#[test]
fn run_all_cases() {
    for (s, words, expected) in CASES.iter().copied() {
        assert_eq!(
            Solution::num_matching_subseq(
                s.to_string(),
                words.into_iter().map(|s| s.to_string()).collect()
            ),
            expected
        );
    }
}
