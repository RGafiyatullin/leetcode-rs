pub struct Solution;

impl Solution {
    pub fn poor_pigs(buckets: i32, mins_to_die: i32, mins_to_test: i32) -> i32 {
        let states = mins_to_test / mins_to_die + 1;
        ((buckets as f64).log2() / (states as f64).log2()).ceil() as i32
    }
}
