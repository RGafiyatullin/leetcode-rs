const CASES: &[(&str, i32)] =
    &[("1 + 1", 2), (" 2-1 + 2", 3), ("(1+(4+5+2)-3)+(6+8)", 23), ("-1", -1), ("(-1 + 2)", 1)];

#[test]
fn run_all_cases() {
    for &(input, exp) in CASES {
        assert_eq!(crate::solution::Solution::calculate(input.to_owned()), exp);
    }
}

#[test]
fn run_0() {
    let &(input, exp) = &CASES[0];
    assert_eq!(crate::solution::Solution::calculate(input.to_owned()), exp);
}

#[test]
fn run_1() {
    let &(input, exp) = &CASES[1];
    assert_eq!(crate::solution::Solution::calculate(input.to_owned()), exp);
}

#[test]
fn run_2() {
    let &(input, exp) = &CASES[2];
    assert_eq!(crate::solution::Solution::calculate(input.to_owned()), exp);
}

#[test]
fn run_3() {
    let &(input, exp) = &CASES[3];
    assert_eq!(crate::solution::Solution::calculate(input.to_owned()), exp);
}
#[test]
fn run_4() {
    let &(input, exp) = &CASES[4];
    assert_eq!(crate::solution::Solution::calculate(input.to_owned()), exp);
}
