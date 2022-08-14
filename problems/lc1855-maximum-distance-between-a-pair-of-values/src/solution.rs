pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        // eprintln!("nums2: {:?}", nums2);

        let index2 = nums2
            .into_iter()
            .enumerate()
            .fold(BTreeMap::new(), |mut acc, (idx, val)| {
                acc.insert(val, idx);
                acc
            });

        // eprintln!("index2: {:?}", index2);

        nums1
            .into_iter()
            .enumerate()
            .flat_map(|(idx1, val1)| {
                // eprintln!("nums1[{:?}]={:?} ...", idx1, val1);
                // eprintln!("  range2: {:?}", index2.range(val1..).collect::<Vec<_>>());
                index2
                    .range(val1..)
                    .next()
                    .filter(|(val2, idx2)| **val2 >= val1 && **idx2 >= idx1)
                    .map(|(_val2, idx2)| *idx2 - idx1)
                // .inspect(|dist| eprintln!("  dist: {:?}", dist))
            })
            .max()
            .unwrap_or_default() as i32
    }
}
