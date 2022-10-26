pub struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        assert!(high >= low);

        let n = high - low + 1;
        match (n % 2, low % 2) {
            (1, 1) => n / 2 + 1,
            _ => n / 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(i32, i32, i32)] =
        &[(0, 0, 0), (1, 1, 1), (1, 2, 1), (1, 3, 2), (2, 3, 1), (2, 4, 1), (3, 7, 3), (8, 10, 1)];

    #[test]
    fn run_all_cases() {
        for (low, high, expected) in CASES {
            assert_eq!(
                Solution::count_odds(*low, *high),
                *expected,
                "low: {}, high: {}, expected: {}",
                low,
                high,
                expected
            );
        }
    }
}
