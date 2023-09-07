macro_rules! test {
    ($name: ident, $nums: expr, $k: expr, $expected: expr) => {
        #[test]
        fn $name() {
            let mut actual = $crate::solution::Solution::top_k_frequent($nums, $k);
            let mut expected = $expected;

            actual.sort();
            expected.sort();

            assert_eq!(actual, expected);
        }
    };
}

test!(example_1, vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]);
test!(example_2, vec![1], 1, vec![1]);

test!(case_1, vec![1, 1, 1, 2, 2, 2, 3, 3, 3], 3, vec![1, 2, 3]);
test!(case_2, vec![4, 1, -1, 2, -1, 2, 3], 2, vec![-1, 2]);
