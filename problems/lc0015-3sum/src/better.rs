pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        use std::ops::Neg;

        nums.sort_unstable();
        let mut nums_1 = &nums[..];

        let mut out = HashSet::new();

        while let Some((&first, tail)) = nums_1.split_first() {
            let mut nums_2 = tail;
            while let Some((&second, tail)) = nums_2.split_first() {
                let third = (first + second).neg();

                if exists(tail, third) {
                    out.insert((first, second, third));
                }
                nums_2 = tail;
            }
            nums_1 = tail;
        }

        out.into_iter().map(|(a, b, c)| vec![a, b, c]).collect()
    }
}

fn exists(sorted: &[i32], num: i32) -> bool {
    use std::cmp::Ordering::*;

    if sorted.is_empty() {
        return false
    }

    let (mut lo, mut hi) = (0, sorted.len().saturating_sub(1));

    loop {
        let idx = (lo + hi) / 2;
        let val = sorted[idx];

        match (lo == hi, val.cmp(&num)) {
            (false, Less) => lo = idx + 1,
            (false, Greater) => hi = idx,
            (_, Equal) => break true,
            (true, Less | Greater) => break false,
        }
    }
}
