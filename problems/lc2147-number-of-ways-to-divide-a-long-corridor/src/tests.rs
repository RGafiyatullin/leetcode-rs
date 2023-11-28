macro_rules! run_test {
    ($case_name: ident, $input: literal, $output: literal) => {
        #[test]
        fn $case_name() {
            assert_eq!(crate::solution::Solution::number_of_ways($input.into()), $output);
        }
    };
}

run_test!(case_1, "SSPPSPS", 3);
run_test!(case_2, "PPSPSP", 1);
run_test!(case_3, "S", 0);
