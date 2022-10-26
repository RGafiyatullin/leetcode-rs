use crate::Solution;

const CASES: &[(&[&str], &str, &[&str])] = &[
    (&["abc", "deq", "mee", "aqq", "dkd", "ccc"], "abb", &["mee", "aqq"]),
    (&["a", "b", "c"], "a", &["a", "b", "c"]),
];

fn run_one_case((input, pattern, expected): (&[&str], &str, &[&str])) {
    let mut expected = expected.to_vec();
    expected.sort();

    let mut actual = Solution::find_and_replace_pattern(
        input.into_iter().copied().map(String::from).collect(),
        pattern.to_owned(),
    );
    actual.sort();

    assert_eq!(actual, expected);
}

#[test]
fn run_all_cases() {
    for &case in CASES.iter() {
        run_one_case(case);
    }
}
