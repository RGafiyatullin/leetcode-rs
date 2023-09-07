macro_rules! test {
    ($name: ident, $input: expr, $expected: expr) => {
        #[test]
        fn $name() {
            let actual =
                $crate::solution::Solution::eval_rpn($input.into_iter().map(Into::into).collect());
            assert_eq!(actual, $expected)
        }
    };
}

test!(example_1, ["2", "1", "+", "3", "*"], 9);
test!(example_2, ["4", "13", "5", "/", "+"], 6);
test!(example_3, ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"], 22);
