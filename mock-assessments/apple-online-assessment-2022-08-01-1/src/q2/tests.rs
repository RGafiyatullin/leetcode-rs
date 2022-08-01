use super::*;

const CASES: &[(i32, &[&str])] = &[
    (1, &["()"]),
    (3, &["((()))", "(()())", "(())()", "()(())", "()()()"]),
];

#[test]
fn run_all_cases() {
    for &(n, exp) in CASES {
        let mut exp = exp.to_vec();
        exp.sort();

        let mut act = Solution::generate_parenthesis(n);
        act.sort();

        assert_eq!(act, exp, "{:?}", n);
    }
}
