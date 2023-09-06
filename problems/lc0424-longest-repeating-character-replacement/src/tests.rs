macro_rules! test {
    ($name: ident, $s: literal, $k: literal, $expected: literal) => {
        #[test]
        fn $name() {
            assert_eq!($crate::solution::Solution::character_replacement($s.into(), $k), $expected)
        }
    };
}

test!(example1, "ABAB", 2, 4);
test!(example2, "AABABBA", 1, 4);

test!(my_case_0_0, "ABBCCCDDDDCCCBBA", 0, 4);
test!(my_case_0_1, "ABBCCCDDDDCCCBBA", 1, 5);
test!(my_case_0_2, "ABBCCCDDDDCCCBBA", 2, 6);
test!(my_case_0_3, "ABBCCCDDDDCCCBBA", 3, 7);
test!(my_case_0_4, "ABBCCCDDDDCCCBBA", 4, 10);
test!(my_case_0_5, "ABBCCCDDDDCCCBBA", 5, 11);
test!(my_case_0_6, "ABBCCCDDDDCCCBBA", 6, 12);
test!(my_case_0_7, "ABBCCCDDDDCCCBBA", 7, 13);
test!(my_case_0_8, "ABBCCCDDDDCCCBBA", 8, 14);
test!(my_case_0_9, "ABBCCCDDDDCCCBBA", 9, 15);
test!(my_case_0_10, "ABBCCCDDDDCCCBBA", 10, 16);
test!(my_case_0_11, "ABBCCCDDDDCCCBBA", 11, 16);
test!(my_case_0_12, "ABBCCCDDDDCCCBBA", 12, 16);
test!(my_case_0_13, "ABBCCCDDDDCCCBBA", 13, 16);
test!(my_case_0_14, "ABBCCCDDDDCCCBBA", 14, 16);
test!(my_case_0_15, "ABBCCCDDDDCCCBBA", 15, 16);
test!(my_case_0_16, "ABBCCCDDDDCCCBBA", 16, 16);

test!(my_case_1_2, "ABCADEFAAGAHI", 2, 5);
