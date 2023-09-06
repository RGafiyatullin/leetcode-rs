macro_rules! test {
    ($test_name: ident, $input: expr, $expected: expr) => {
        #[test]
        fn $test_name() {
            assert_eq!($crate::solution::Solution::spiral_order($input), $expected);
        }
    };
}

test!(
    example_1,
    vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
    vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
);
test!(
    example_2,
    vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
    vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
);
