pub struct Solution;

use std::collections::BTreeMap;

const RETENTION_MOD: i32 = 100;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut out = vec![0; nums.len()];

        let mut stats_fine = BTreeMap::<i32, i32>::new();
        let mut stats_coarse = BTreeMap::<i32, i32>::new();

        for (idx, value_fine) in nums.into_iter().enumerate().rev() {
            let value_coarse = match value_fine % RETENTION_MOD {
                neg if neg < 0 => value_fine / RETENTION_MOD - 1,
                _ => value_fine / RETENTION_MOD,
            } * RETENTION_MOD;

            let bounds_fine = value_coarse..value_fine;
            let bounds_coarse = ..value_coarse - 1;

            // eprintln!("IDX: {}", idx);
            // eprintln!(" value-fine:   {:?}", value_fine);
            // eprintln!(" value-coarse: {:?}", value_coarse);

            // eprintln!(" // bounds-fine: {:?}", bounds_fine);
            // eprintln!(" // bounds-coarse: {:?}", bounds_coarse);

            out[idx] = stats_fine
                .range(bounds_fine.clone())
                .map(|(_, v)| *v)
                .sum::<i32>()
                + stats_coarse
                    .range(bounds_coarse.clone())
                    .map(|(_, v)| *v)
                    .sum::<i32>();

            // eprintln!(" out: {:?}", out[idx]);

            *stats_fine.entry(value_fine).or_default() += 1;
            *stats_coarse.entry(value_coarse).or_default() += 1;
        }

        out
    }
}
