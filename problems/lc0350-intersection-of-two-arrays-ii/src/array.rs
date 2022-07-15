pub struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut rc = [0 as usize; 1001];

        nums1.into_iter().for_each(|item| {
            rc[item as usize] += 1;
        });

        nums2
            .into_iter()
            .filter(|item| {
                if rc[*item as usize] > 0 {
                    rc[*item as usize] -= 1;
                    true
                } else {
                    false
                }
            })
            .collect()
    }
}