use crate::Solution;

const CASES: &[(&str, &str, &str, bool)] = &[
    ("", "", "", true),
    ("aabcc", "dbbca", "aadbbcbcac", true),
    ("aabcc", "dbbca", "aadbbbaccc", false),
];

#[test]
fn all_cases() {
    for &case in CASES {
        run_one_case(case);
    }
}

#[test]
fn case_0() {
    run_one_case(CASES[0]);
}

fn run_one_case(case: (&str, &str, &str, bool)) {
    let (s1, s2, s3, expected) = case;
    let actual = Solution::is_interleave(s1.to_string(), s2.to_string(), s3.to_string());
    assert_eq!(actual, expected, "s1: {}, s2: {}, s3: {}", s1, s2, s3);
}
