pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (smaller, larger) =
            if nums1.len() > nums2.len() { (nums1, nums2) } else { (nums2, nums1) };

        let mut map = smaller.into_iter().fold(HashMap::new(), |mut acc, item| {
            *acc.entry(item).or_insert(0) += 1;
            acc
        });

        larger
            .into_iter()
            .filter(|&item| {
                if let Some(count) = map.get_mut(&item) {
                    if *count > 0 {
                        *count -= 1;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .collect()
    }
}
