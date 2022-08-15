use crate::Solution;

const CASES: &[(&str, i32)] = &[("III", 3), ("LVIII", 58), ("MCMXCIV", 1994)];

#[test]
fn run_all_cases() {
    for &(input, exp) in CASES {
        eprintln!("{:?} -> {:?}", input, exp);
        assert_eq!(Solution::roman_to_int(input.to_owned()), exp);
    }
}
