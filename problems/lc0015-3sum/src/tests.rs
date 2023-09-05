macro_rules! test {
    ($test_name: ident, $input: expr, $expected: expr) => {
        #[test]
        fn $test_name() {
            let input = $input;
            let mut expected = $expected;
            let mut actual = $crate::solution::Solution::three_sum(input);

            expected.sort();
            actual.sort();

            assert_eq!(actual, expected);
        }
    };
}

test!(example_1, vec![-1, 0, 1, 2, -1, -4], vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
test!(example_2, vec![0, 1, 1], Vec::<Vec<i32>>::new());
test!(example_3, vec![0, 0, 0], vec![vec![0, 0, 0]]);
