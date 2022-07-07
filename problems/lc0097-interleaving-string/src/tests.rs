use crate::Solution;

const CASES: &[(&str, &str, &str, bool)] = &[
    ("", "", "", true),
    ("aabcc", "dbbca", "aadbbcbcac", true),
    ("aabcc", "dbbca", "aadbbbaccc", false),

    (
        "bbbbbabbbbabaababaaaabbababbaaabbabbaaabaaaaababbbababbbbbabbbbababbabaabababbbaabababababbbaaababaa",
        "babaaaabbababbbabbbbaabaabbaabbbbaabaaabaababaaaabaaabbaaabaaaabaabaabbbbbbbbbbbabaaabbababbabbabaab",
        "babbbabbbaaabbababbbbababaabbabaabaaabbbbabbbaaabbbaaaaabbbbaabbaaabababbaaaaaabababbababaababbababbbababbbbaaaabaabbabbaaaaabbabbaaaabbbaabaaabaababaababbaaabbbbbabbbbaabbabaabbbbabaaabbababbabbabbab",
        false,
    )
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

#[test]
fn case_1() {
    run_one_case(CASES[1]);
}

#[test]
fn case_2() {
    run_one_case(CASES[2]);
}

#[test]
fn case_3() {
    run_one_case(CASES[3]);
}

fn run_one_case(case: (&str, &str, &str, bool)) {
    let (s1, s2, s3, expected) = case;
    let actual = Solution::is_interleave(s1.to_string(), s2.to_string(), s3.to_string());
    assert_eq!(actual, expected, "s1: {}, s2: {}, s3: {}", s1, s2, s3);
}
