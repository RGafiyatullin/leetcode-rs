use crate::Solution;

const CASES: &[(i32, i32)] = &[(1, 5), (2, 10), (5, 68), (1024, 245633651), (20000, 759959057)];

#[test]
fn run_all_cases() {
    for &(n, exp) in CASES {
        assert_eq!(Solution::count_vowel_permutation(n), exp);
    }
}
