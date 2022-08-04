#![allow(non_snake_case, dead_code)]

pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let n = n as i64;

        let (mut lo, mut hi) = (1, n);

        loop {
            let mid = (lo + hi) / 2;
            match guess(mid as i32).cmp(&0) {
                Ordering::Less => {
                    hi = mid - 1;
                }
                Ordering::Greater => {
                    lo = mid + 1;
                }
                Ordering::Equal => break mid as i32,
            }
        }
    }
}

const PICKED: i32 = 1702766719;
const MAX: i32 = 2126753390;

unsafe fn guess(num: i32) -> i32 {
    match num.cmp(&PICKED) {
        Ordering::Equal => 0,
        Ordering::Greater => -1,
        Ordering::Less => 1,
    }
}

#[test]
fn test() {
    assert_eq!(unsafe { Solution::guessNumber(MAX) }, PICKED);
}
