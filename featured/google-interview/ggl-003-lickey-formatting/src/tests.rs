use crate::Solution;

const CASES: &[(i32, &str, &str)] = &[(4, "5F3Z-2e-9-w", "5F3Z-2E9W"), (2, "2-5g-3-J", "2-5G-3J")];

#[test]
fn run_all_cases() {
    for (k, input, expected) in CASES {
        assert_eq!(
            Solution::license_key_formatting(input.to_string(), *k).as_str(),
            *expected
        );
    }
}
