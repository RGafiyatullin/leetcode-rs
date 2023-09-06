macro_rules! test {
    ($test_name: ident, $input: expr, $expected: expr) => {
        #[test]
        fn $test_name() {
            let actual = $crate::solution::Solution::num_decodings($input.into());
            assert_eq!(actual, $expected);
        }
    };
}

test!(empty_string, "", 1);
test!(example_1, "12", 2);
test!(example_2, "226", 3);
test!(example_3, "06", 0);
