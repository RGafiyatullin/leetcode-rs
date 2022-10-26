use crate::Solution;

#[test]
fn seqs_of_equal_len_are_sorted_lexicographically() {
    assert!([2, 1, 3] < [3, 1, 2]);
}

const CASES: &[(&str, &[i32])] = &[("I", &[1, 2]), ("DI", &[2, 1, 3]), ("IDDI", &[3, 4, 2, 1, 5])];

#[test]
fn run_all_cases() {
    for &(input, exp) in CASES {
        assert_eq!(Solution::find_permutation(input.to_owned()), exp);
    }
}
