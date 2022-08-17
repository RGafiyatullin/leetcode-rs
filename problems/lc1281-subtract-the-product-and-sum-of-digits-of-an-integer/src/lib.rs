pub struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        assert!(n >= 1);
        assert!(n <= 100_000);

        const RADIX: i32 = 10;

        let mut product = 1;
        let mut sum = 0;

        while n != 0 {
            let digit = n.rem_euclid(RADIX);
            n = n.div_euclid(RADIX);

            product *= digit;
            sum += digit;
        }

        product - sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(i32, i32)] = &[(234, 15), (4421, 21)];

    #[test]
    fn run_all_cases() {
        for &(n, exp) in CASES {
            assert_eq!(Solution::subtract_product_and_sum(n), exp);
        }
    }
}
