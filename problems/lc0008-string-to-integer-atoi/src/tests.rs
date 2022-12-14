const CASES: &[(&str, i32)] =
    &[("42", 42), ("   -42", -42), ("4193 with words", 4193), ("should be zero", 0)];

#[test]
fn run_all_cases() {
    for &(input, expected) in CASES {
        assert_eq!(
            crate::solution::Solution::my_atoi(input.to_owned()),
            expected,
            "{} => {}",
            input,
            expected
        );
    }
}
