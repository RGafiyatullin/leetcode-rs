pub struct Solution;

impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}
