pub struct Solution;

const RADIX: i32 = 10;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_negative = x.is_negative();

        let mut input: i32 = x;
        let mut output: i32 = 0;

        while input != 0 {
            if let Some(o) = output.checked_mul(RADIX) {
                output = o;
            } else {
                return 0
            };

            let rem = input % RADIX;
            input /= RADIX;

            if let Some(o) = output.checked_add(rem) {
                output = o;
            } else {
                return 0
            };
        }

        output
    }
}

#[test]
fn basics() {
    assert_eq!(11 % 10, 1);
    assert_eq!(-11 % -10, -1);
    assert_eq!(-11 % 10, -1);
}
