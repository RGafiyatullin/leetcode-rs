macro_rules! test {
    ($name: ident, $input: expr, $expected: expr) => {
        #[test]
        fn $name() {
            let mut actual = $crate::solution::Solution::generate_parenthesis($input);
            let mut expected = $expected.into_iter().map(String::from).collect::<Vec<_>>();
            actual.sort_unstable();
            expected.sort_unstable();
            assert_eq!(actual, expected);
        }
    };
}

test!(example_1, 3, ["((()))", "(()())", "(())()", "()(())", "()()()"]);
test!(example_2, 1, ["()"]);

test!(my_case_1, 1, ["()"]);
test!(my_case_2, 2, ["()()", "(())"]);
test!(my_case_3, 3, ["(()())", "((()))", "()()()", "()(())", "(())()"]);
