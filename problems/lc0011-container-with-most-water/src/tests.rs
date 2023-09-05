macro_rules! test {
    ($test_name: ident, $input: expr, $expected: literal) => {
        #[test]
        fn $test_name() {
            assert_eq!($crate::solution::Solution::max_area($input.to_vec()), $expected);
        }
    };
}

test!(example_1, [1,8,6,2,5,4,8,3,7], 49);
test!(example_2, [1,1], 1);
