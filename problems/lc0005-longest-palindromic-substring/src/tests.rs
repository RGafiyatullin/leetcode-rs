macro_rules! test {
    ($test_name: ident, $input: literal, $expected: literal) => {
        #[test]
        fn $test_name() {
            assert_eq!(
                $crate::solution::Solution::longest_palindrome($input.to_owned()),
                $expected
            );
        }
    };
}

test!(example_01, "babad", "bab");
test!(example_02, "cbbd", "bb");
