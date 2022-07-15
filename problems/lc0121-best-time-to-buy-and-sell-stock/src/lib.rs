
pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut last_min = None;
        for price in prices {
            last_min = Some(last_min.map(|min| std::cmp::min(min, price)).unwrap_or(price));

            if let Some(min) = last_min {
                max_profit = std::cmp::max(max_profit, price - min);
            }
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    const CASES: &[(&[i32], i32)] = &[
        (&[7,1,5,3,6,4], 5),
        (&[7,6,4,3,1], 0),
    ];

    #[test]
    fn test_max_profit() {
        for &(prices, expected) in CASES {
            assert_eq!(super::Solution::max_profit(prices.to_vec()), expected);
        }
    }
}