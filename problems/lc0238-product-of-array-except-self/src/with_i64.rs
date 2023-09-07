pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let total_count = nums.len();

        let mut zeroes_count = 0;
        let mut product = 1;
        let mut last_zero_idx = 0;

        for (i, n) in nums.iter().copied().enumerate() {
            if n == 0 {
                zeroes_count += 1;
                last_zero_idx = i;
            } else {
                product *= n as i64;
            }
        }

        // dbg!(total_count, zeroes_count, product, last_zero_idx);

        let out = match (total_count, zeroes_count) {
            (0, _) => vec![],
            (_, 0) => {
                let mut out = vec![0; total_count];
                out[total_count - 1] = product / nums[total_count - 1] as i64;

                for i in (0..(total_count - 1)).rev() {
                    out[i] = out[i + 1];
                    out[i] /= nums[i] as i64;
                    out[i] *= nums[i + 1] as i64;
                    // dbg!(i);
                    // dbg!(out[i + 1]);
                    // dbg!(out[i]);
                }
                out
            },
            (_, nz) => {
                let mut out = vec![0; total_count];
                if nz == 1 {
                    out[last_zero_idx] = product;
                }
                out
            },
        };
        out.into_iter().map(|i| i as i32).collect()
    }
}
