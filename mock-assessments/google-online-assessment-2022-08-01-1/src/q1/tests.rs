use super::*;

const CASES: &[(&str, &str)] =
    &[("()", ""), ("(()())(())", "()()()"), ("(()())(())(()(()))", "()()()()(())"), ("()()", "")];

#[test]
fn run_all_cases() {
    for &(input, expected) in CASES {
        let actual = Solution::remove_outer_parentheses(input.to_owned());

        assert_eq!(actual, expected, "CASE: {:?}", input);
    }
}
