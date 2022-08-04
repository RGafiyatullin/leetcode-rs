pub struct Solution;

impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let p = p as i64;
        let q = q as i64;

        let mut pos = 0;
        let mut left_side = true;

        loop {
            pos += q;
            pos %= 2 * p;
            left_side = !left_side;

            match (pos, left_side) {
                (0, false) => break 0,
                (eq_p, false) if eq_p == p => break 1,
                (eq_p, true) if eq_p == p => break 2,
                (0, true) => break 3,
                (_, _) => continue,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(i32, i32, i32)] = &[
        (7, 2, 0),
        (7, 3, 1),
        (7, 5, 1),
        (1000, 999, 2),
        // (100000000, 99999999, 2),
    ];

    #[test]
    fn test() {
        for &(p, q, exp) in CASES {
            assert_eq!(Solution::mirror_reflection(p, q), exp);
        }
    }
}
