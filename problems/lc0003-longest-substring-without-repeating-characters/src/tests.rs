const CASES: &[(&str, usize)] = &[("abcabcbb", 3), ("bbbbb", 1), ("pwwkew", 3), ("tmmzuxt", 5)];

#[test]
fn run_all_cases() {
    for &(s, exp) in CASES {
        assert_eq!(
            crate::solution::Solution::length_of_longest_substring(s.to_owned()) as usize,
            exp
        );
    }
}

#[test]
fn run_tmmzuxt_5() {
    assert_eq!(crate::solution::Solution::length_of_longest_substring("tmmzuxt".to_owned()), 5)
}
