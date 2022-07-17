pub struct Solution;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;

        const MODULO: u64 = 1_000_000_007;

        let mut prev = vec![0; k as usize + 1];
        let mut curr = vec![0; k as usize + 1];

        for i in 1..=n {
            curr[0] = 1;

            for j in 1..=k {
                let mut val = prev[j] + MODULO;
                if j >= i { val -= prev[j - i]; }
                val %= MODULO;

                curr[j] = curr[j - 1];
                curr[j] += val;
                curr[j] %= MODULO;
            }

            std::mem::swap(&mut prev, &mut curr);
        }

        let mut ret = prev[k] + MODULO;
        if k > 0 { ret -= prev[k - 1]; }
        ret %= MODULO;

        (ret % MODULO) as i32
    }
}
