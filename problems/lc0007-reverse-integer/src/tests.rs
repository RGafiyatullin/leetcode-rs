macro_rules! test {
    ($test_name: ident, $input: literal, $expected: literal) => {
        #[test]
        fn $test_name() {
            assert_eq!($crate::solution::Solution::reverse($input), $expected);
        }
    };
}

test!(example_1, 123, 321);
test!(example_2, -123, -321);
test!(example_3, 120, 21);
