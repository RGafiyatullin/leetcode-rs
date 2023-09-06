macro_rules! test {
    ($name: ident, $input: expr, $expected: expr) => {
        #[test]
        fn $name() {
            let actual = crate::solution::Solution::is_palindrome($input.into());
            assert_eq!(actual, $expected);
        }
    };
}

test!(example_1, "A man, a plan, a canal: Panama", true);
test!(example_2, "race a car", false);
test!(example_3, " ", true);

test!(digits_expect_false, "0P", false);
test!(digits_expect_true, "A0A0A", true);

test!(digits_and_comma_expect_false, "9,8", false);

#[test]
fn digits_as_chars() {
    assert_eq!(('0'..='9').collect::<String>(), "0123456789")
}
