const CASES: &[(&str, &str, &[&str], i32)] = &[
    ("AACCGGTT", "AACCGGTA", &["AACCGGTA"], 1),
    ("AACCGGTT", "AAACGGTA", &["AACCGGTA", "AACCGCTA", "AAACGGTA"], 2),
    ("AAAAACCC", "AACCCCCC", &["AAAACCCC", "AAACCCCC", "AACCCCCC"], 3),
];

#[test]
fn all_cases() {
    for &(start, end, bank, exp) in CASES {
        assert_eq!(
            crate::solution::Solution::min_mutation(
                start.to_owned(),
                end.to_owned(),
                bank.into_iter().copied().map(ToOwned::to_owned).collect()
            ),
            exp
        );
    }
}
