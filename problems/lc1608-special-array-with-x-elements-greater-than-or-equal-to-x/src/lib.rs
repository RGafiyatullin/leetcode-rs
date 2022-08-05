pub struct Solution;

use std::cmp::Ordering::{self, *};

impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 1000);

        nums.sort_by(|l, r| r.cmp(l));

        let (mut lo, mut hi) = (1, nums.len());

        let found = loop {
            let candidate = (lo + hi) / 2;

            match (lo == hi, check(&nums, candidate)) {
                (false, Less) => hi = candidate,
                (false, Greater) => lo = candidate + 1,
                (_, Equal) => break Some(candidate),
                (true, Less | Greater) => break None,
            }
        };

        found.map(|n| n as i32).unwrap_or(-1)
    }
}

fn check(nums: &[i32], candidate: usize) -> Ordering {
    let gte_slice = &nums[0..candidate];
    let lt_slice = &nums[candidate..];

    // eprintln!(" Trying {:?} >= {:?} > {:?}", gte_slice, candidate, lt_slice);

    let gte_opt = gte_slice.last();
    let lt_opt = lt_slice.first();

    if lt_opt.into_iter().all(|x| *x < candidate as i32)
        && gte_opt.into_iter().all(|x| *x >= candidate as i32)
    {
        Ordering::Equal
    } else if lt_opt.into_iter().any(|x| *x >= candidate as i32) {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_sort() {
        let mut nums = vec![1, 2, 3, 4, 5, 6];
        nums.sort_by(|l, r| r.cmp(l));
        assert_eq!(nums, &[6, 5, 4, 3, 2, 1]);
    }

    const CASES: &[(&[i32], i32)] = &[
        (&[3, 5], 2),
        (&[0, 0], -1),
        (&[0, 4, 3, 0, 4], 3),
        (&[2, 2, 2, 3, 4, 4, 5, 5, 5, 6, 7], 5),
        (&[2, 2, 2, 3, 4, 4, 6, 6, 6, 6, 7], 5),
    ];

    #[test]
    fn run_all_cases() {
        for &(nums, exp) in CASES {
            eprintln!("{:?} => {:?}", nums, exp);
            assert_eq!(Solution::special_array(nums.to_vec()), exp);
        }
    }
}
