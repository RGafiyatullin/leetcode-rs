pub mod array;
pub mod hashtable;

#[cfg(test)]
mod tests {
    use super::hashtable::Solution;

    const CASES: &[(&[i32], &[i32], &[i32])] = &[
        (&[1, 2, 2, 1], &[2, 2], &[2, 2]),
        (&[4, 9, 5], &[9, 4, 9, 8, 4], &[9, 4]),
    ];

    #[test]
    fn run_all_cases() {
        for (left, right, expected) in CASES {
            let mut expected = expected.to_vec();
            expected.sort();

            let mut out = Solution::intersect(left.to_vec(), right.to_vec());
            out.sort();

            assert_eq!(out, expected);
        }
    }
}