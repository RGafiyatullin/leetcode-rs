pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, mut idx1: i32, nums2: &mut Vec<i32>, mut idx2: i32) {
        assert_eq!(nums1.len(), (idx1 + idx2) as usize);
        assert_eq!(nums2.len(), idx2 as usize);

        idx1 -= 1;
        idx2 -= 1;

        for dst_idx in (0..nums1.len()).rev() {
            if idx2 < 0 || (idx1 >= 0 && nums1[idx1 as usize] >= nums2[idx2 as usize]) {
                nums1[dst_idx] = nums1[idx1 as usize];
                idx1 = idx1.saturating_sub(1);
            } else {
                nums1[dst_idx] = nums2[idx2 as usize];
                idx2 = idx2.saturating_sub(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_01() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];

        Solution::merge(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(nums1, &[1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn case_02() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];

        Solution::merge(&mut nums1, 0, &mut nums2, 1);

        assert_eq!(nums1, &[1]);
    }
}
