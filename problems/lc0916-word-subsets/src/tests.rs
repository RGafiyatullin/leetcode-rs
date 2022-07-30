use crate::Solution;

const CASES: &[(&[&str], &[&str], &[&str])] = &[
    (
        &["amazon", "apple", "facebook", "google", "leetcode"],
        &["e", "o"],
        &["facebook", "google", "leetcode"],
    ),
    (
        &["amazon", "apple", "facebook", "google", "leetcode"],
        &["l", "e"],
        &["apple", "google", "leetcode"],
    ),
    // (
    //     &["amazon", "apple", "facebook", "google", "leetcode"],
    //     &["e", "o", "i"],
    //     &["facebook", "google"],
    // ),
];

fn run_one_case((input, matches, expected): (&[&str], &[&str], &[&str])) {
    let input = input.into_iter().copied().map(String::from).collect();
    let matches = matches.into_iter().copied().map(String::from).collect();
    let mut expected = expected.to_owned();
    expected.sort();

    let mut actual = Solution::word_subsets(input, matches);
    actual.sort();

    assert_eq!(expected, actual);
}

#[test]
fn run_all_tests() {
    for &case in CASES {
        run_one_case(case);
    }
}
