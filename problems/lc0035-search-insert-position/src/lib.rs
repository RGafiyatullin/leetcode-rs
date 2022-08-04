pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        eprintln!("{:?} | {:?}", nums, target);
        let (mut lo, mut hi) = (0 as usize, nums.len() - 1);

        let idx = loop {
            eprintln!("[{:?}..{:?}]", lo, hi);
            assert!(lo <= hi);

            let mid_idx = (lo + hi) / 2;
            let mid_val = nums[mid_idx];

            eprintln!(" values[{:?}] = {:?}", mid_idx, mid_val);

            match (mid_val.cmp(&target), lo == hi) {
                (Ordering::Equal, _) => break mid_idx,
                (Ordering::Greater, true) => break mid_idx,
                (Ordering::Less, true) => break mid_idx + 1,

                (Ordering::Greater, false) => hi = mid_idx,
                (Ordering::Less, false) => lo = mid_idx + 1,
            }
        };

        idx as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    const CASES: &[(&[i32], i32, i32)] = &[
        (&[1, 3, 5, 6], 5, 2),
        (&[1, 3, 5, 6], 2, 1),
        (&[1, 3, 5, 6], 7, 4),
        (&[1, 2], 0, 0),
        (&[3, 5, 7, 9, 10], 8, 3),
    ];

    #[test]
    fn run_all_cases() {
        for &(input, target, exp) in CASES {
            assert_eq!(Solution::search_insert(input.to_vec(), target), exp);
        }
    }
}
