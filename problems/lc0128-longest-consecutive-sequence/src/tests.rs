macro_rules! test {
    ($name: ident, $input: expr, $expected: expr) => {
        #[test]
        fn $name() {
            let actual = $crate::solution::Solution::longest_consecutive($input);
            assert_eq!(actual, $expected);
        }
    };
}

test!(example_1, vec![100, 4, 200, 1, 3, 2], 4);
test!(example_2, vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9);
