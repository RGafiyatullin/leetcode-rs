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
                return distinct
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
