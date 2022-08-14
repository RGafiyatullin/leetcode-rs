pub struct Solution;

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        nums1
            .into_iter()
            .enumerate()
            .flat_map(|(i1, v1)| {
                nums2
                    .iter()
                    .copied()
                    .enumerate()
                    .filter(move |(i2, v2)| i1 <= *i2 && v1 <= *v2)
                    .map(move |(i2, _)| i2 - i1)
            })
            .max()
            .unwrap_or_default() as i32
    }
}
