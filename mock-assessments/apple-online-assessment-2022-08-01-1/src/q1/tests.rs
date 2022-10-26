use super::*;

const CASES: &[(&[&str], &[&str])] = &[
    (&[], &[]),
    (
        &["dig1 8 1 5 1", "let1 art can", "dig2 3 6", "let2 own kit dig", "let3 art zero"],
        &["let1 art can", "let3 art zero", "let2 own kit dig", "dig1 8 1 5 1", "dig2 3 6"],
    ),
    (
        &["a1 9 2 3 1", "g1 act car", "zo4 4 7", "ab1 off key dog", "a8 act zoo"],
        &["g1 act car", "a8 act zoo", "ab1 off key dog", "a1 9 2 3 1", "zo4 4 7"],
    ),
];

#[test]
fn run_all_cases() {
    for &(input, expected) in CASES {
        assert_eq!(
            Solution::reorder_log_files(input.into_iter().copied().map(String::from).collect()),
            expected
        );
    }
}
