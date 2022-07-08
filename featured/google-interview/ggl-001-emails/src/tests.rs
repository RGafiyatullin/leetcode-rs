use crate::Solution;

const CASES: &[(&[&str], i32)] = &[
    (
        &[
            "test.email+alex@leetcode.com",
            "test.e.mail+bob.cathy@leetcode.com",
            "testemail+david@lee.tcode.com",
        ],
        2,
    ),
    (&["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"], 3),
];

#[test]
fn run_all_cases() {
    for (input, expected) in CASES {
        assert_eq!(
            Solution::num_unique_emails(input.iter().map(|s| s.to_string()).collect()),
            *expected
        );
    }
}
