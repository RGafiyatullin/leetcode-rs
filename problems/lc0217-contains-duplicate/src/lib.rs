pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut occurrences = HashSet::new();
        nums.into_iter().any(|n| !occurrences.insert(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&[i32], bool)] = &[
        (&[1], false),
        (&[1, 2], false),
        (&[1, 1], true),
        (&[1, 2, 3, 1], true),
    ];

    #[test]
    fn run_all_cases() {
        for case in CASES {
            assert_eq!(
                Solution::contains_duplicate(case.0.to_vec()),
                case.1,
                "case: {:?}",
                case
            );
        }
    }
}
