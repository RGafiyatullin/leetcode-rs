macro_rules! test {
    ($name: ident, $input: expr, $expected: expr) => {
        #[test]
        fn $name() {
            let actual = $crate::solution::Solution::product_except_self($input);
            assert_eq!(actual, $expected);
        }
    };
}

test!(example_1, vec![1, 2, 3, 4], vec![24, 12, 8, 6]);
test!(example_2, vec![-1, 1, 0, -3, 3], vec![0, 0, 9, 0, 0]);

test!(my_case_1, vec![1, 2, 0, 0, 3, 4], vec![0, 0, 0, 0, 0, 0]);

test!(
    my_case_2,
    vec![0x100, 0x100, 0x100, 0x100],
    vec![0x1000000, 0x1000000, 0x1000000, 0x1000000]
);
