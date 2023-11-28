macro_rules! a_test {
    ($case_name: ident, $input: literal, $output: literal) => {
        #[test]
        fn $case_name() {
            assert_eq!(crate::solution::Solution::knight_dialer($input), $output);
        }
    };
}

a_test!(t_1, 1, 10);
a_test!(t_2, 2, 20);
a_test!(t_3131, 3131, 136006598);
