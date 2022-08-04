pub struct Solution;

#[cfg(feature = "naive")]
impl Solution {
    pub fn arrange_coins(mut n: i32) -> i32 {
        assert!(n >= 1);

        (1..)
            .skip_while(|r| {
                n -= *r;
                n >= 0
            })
            .next()
            .map(|r| r - 1)
            .expect("Come on!")
    }
}

#[cfg(feature = "triangular-number")]
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        use std::cmp::Ordering::*;

        assert!(n >= 1);

        let available_coins = n as u64;

        let (mut lo, mut hi) = (0, available_coins);

        let out = loop {
            let complete_rows = (lo + hi) / 2;
            let required_coins = complete_rows * (complete_rows + 1) / 2;

            match (lo == hi, required_coins.cmp(&available_coins)) {
                (false, Less) => lo = complete_rows + 1,
                (false, Greater) => hi = complete_rows,
                (_, Equal) => break complete_rows,
                (true, _) => break complete_rows - 1,
            }
        };

        out as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for (n, expected) in [(5, 2), (8, 3), (1, 1)] {
            eprintln!(">> {} | {}", n, expected);
            assert_eq!(Solution::arrange_coins(n), expected);
        }
    }
}
