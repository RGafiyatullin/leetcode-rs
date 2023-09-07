macro_rules! test {
    ($name: ident, $input: expr, $expected: expr) => {
        #[test]
        fn $name () {
            let actual = $crate::solution::Solution::generate_parenthesis($input);
            let expected = $expected.into_iter().map(String::from).collect::<Vec<_>>();
            assert_eq!(actual, expected);
        }
    };
}

test!(example_1, 3, ["((()))","(()())","(())()","()(())","()()()"]);
test!(example_2, 1, ["()"]);