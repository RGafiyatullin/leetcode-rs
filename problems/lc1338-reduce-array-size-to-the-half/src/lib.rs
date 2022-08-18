pub struct Solution;

use std::collections::BinaryHeap;

const MIN_VAL: i32 = 1;
const MAX_VAL: i32 = 100_000;
const DOMAIN_SIZE: usize = (MAX_VAL - MIN_VAL + 1) as usize;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        assert!(arr.len() >= 2);
        assert!(arr.len() <= 100_000);
        assert!(arr.len() % 2 == 0);

        let target = arr.len() as u32 / 2;

        let mut heap = arr
            .into_iter()
            .fold(vec![0 as u32; DOMAIN_SIZE], |mut acc, val| {
                acc[idx(val)] += 1;
                acc
            })
            .into_iter()
            .filter(|q| *q > 0)
            .fold(BinaryHeap::new(), |mut acc, q| {
                acc.push(q);
                acc
            });

        let mut total = 0;
        let mut distinct = 0;

        while let Some(q) = heap.pop() {
            total += q;
            distinct += 1;
            if total >= target {
                return distinct;
            }
        }

        unreachable!("The sum of all items of the `heap` is expected to be equal to `arr.len()`, which is twice more than `target`. QED")
    }
}

fn idx(val: i32) -> usize {
    assert!(val >= MIN_VAL);
    assert!(val <= MAX_VAL);

    (val - MIN_VAL) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&[i32], i32)] = &[
        (&[3, 3, 3, 3, 5, 5, 5, 2, 2, 7], 2),
        (&[7, 7, 7, 7, 7, 7], 1),
        (
            &[
                72, 18, 36, 6, 12, 97, 41, 82, 44, 75, 82, 42, 75, 48, 63, 9, 61, 50, 11, 91, 24,
                26, 3, 65, 31, 67, 15, 76, 54, 59, 4, 27, 33, 26, 17, 60, 100, 19, 90, 6, 80, 82,
                48, 70, 85, 99, 69, 2, 78, 94, 15, 29, 75, 17, 9, 79, 99, 88, 26, 73, 88, 100, 9,
                95, 2, 56, 63, 31, 25, 40, 8, 100, 56, 44, 36, 42, 21, 96, 63, 38, 96, 78, 60, 22,
                21, 81,
            ],
            19,
        ),
    ];

    #[test]
    fn run_all_test() {
        for &(input, exp) in CASES {
            eprintln!("{:?} -> {:?}", input, exp);
            assert_eq!(Solution::min_set_size(input.to_vec()), exp);
        }
    }
}
