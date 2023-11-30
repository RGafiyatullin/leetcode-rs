pub struct Solution;

impl Solution {
    pub fn minimum_one_bit_operations(mut n: i32) -> i32 {
        for i in [16, 8, 4, 2, 1] {
            n ^= n >> i;
        }
        n
    }
}

