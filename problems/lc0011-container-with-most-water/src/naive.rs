pub struct Solution;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max = 0;
        for l in 0..heights.len() {
            for r in l..heights.len() {
                let h = heights[l].min(heights[r]);
                let w = (r - l) as i32;

                max = max.max(w * h);
            }
        }

        max
    }
}
