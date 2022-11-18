pub struct Solution;

impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n < 1 {
            false
        } else {
            const FACTORS: &[i32] = &[2, 3, 5];

            for &factor in FACTORS {
                while n % factor == 0 {
                    n /= factor;
                }
            }

            n == 1
        }
    }
}
