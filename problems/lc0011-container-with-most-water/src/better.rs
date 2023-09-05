pub struct Solution;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = heights.len() - 1;

        let mut max = 0;

        while l < r {
            let w = (r - l) as i32;
            let h = heights[l].min(heights[r]);
            max = max.max(w * h);

            if heights[l] < heights[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        max
    }
}
