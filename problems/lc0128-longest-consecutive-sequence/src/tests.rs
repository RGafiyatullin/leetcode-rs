macro_rules! test {
    ($impl:ident, $name: ident, $input: expr, $expected: expr) => {
        #[test]
        fn $name() {
            let actual = $crate::$impl::Solution::longest_consecutive($input);
            assert_eq!(actual, $expected);
        }
    };
}

mod sort_inplace {
    test!(sort_inplace, example_1, vec![100, 4, 200, 1, 3, 2], 4);
    test!(sort_inplace, example_2, vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9);

    test!(sort_inplace, my_case_1, vec![100, 4, 4, 200, 1, 1, 3, 3, 2, 2], 4);

    test!(sort_inplace, case_1, vec![0], 1);
}

mod tree {
    test!(tree, example_1, vec![100, 4, 200, 1, 3, 2], 4);
    test!(tree, example_2, vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9);

    test!(tree, my_case_1, vec![100, 4, 4, 200, 1, 1, 3, 3, 2, 2], 4);

    test!(tree, case_1, vec![0], 1);
}

mod linear_time {
    test!(linear_time, example_1, vec![100, 4, 200, 1, 3, 2], 4);
    test!(linear_time, example_2, vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9);

    test!(linear_time, my_case_1, vec![100, 4, 4, 200, 1, 1, 3, 3, 2, 2], 4);

    test!(linear_time, case_1, vec![0], 1);
}
