
pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let count = nums.len();
        let count_unique = nums.into_iter().collect::<HashSet<_>>().len();

        count == count_unique
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const CASES: &[(&[i32], bool)] = &[
        (&[1], true),
        (&[1,2], true),
        (&[1,1], false),
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