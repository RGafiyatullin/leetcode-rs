pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        // eprintln!("SQRT({:?}) => ...", x);
        use std::cmp::Ordering::*;

        assert!(x >= 0);

        let x = x as u64;

        let (mut lo, mut hi) = (0, x);

        let out = loop {
            let mid = (lo + hi) / 2;
            // std::thread::sleep(std::time::Duration::from_millis(100));

            // eprintln!("{:?}-{:?}-{:?} | {:?}", lo, mid, hi, (mid * mid).cmp(&x));

            match (lo == hi, (mid * mid).cmp(&x)) {
                (false, Less) => lo = mid + 1,
                (false, Greater) => hi = mid,
                (_, Equal) => break mid,
                (true, _) => break mid - 1,
            }
        };

        out as i32
    }
}

#[test]
fn test_01() {
    for i in 1..1000 {
        let sqr_this = i * i;
        let sqr_next = (i + 1) * (i * 1);

        for sqr in sqr_this..sqr_next {
            assert_eq!(Solution::my_sqrt(sqr), i);
        }
    }
}
