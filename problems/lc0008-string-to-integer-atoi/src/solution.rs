pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut acc: u64 = 0;
        let mut state = State::LeadingSpace;
        let mut is_negative = false;

        for ch in s.chars() {
            match (ch, state) {
                (' ', State::LeadingSpace) => continue,
                (digit, State::LeadingSpace | State::AfterSign) if digit.is_numeric() => {
                    acc *= 10;
                    acc += (digit as u8 - '0' as u8) as u64;

                    if is_negative && acc > (i32::MIN as i64).abs() as u64 {
                        return i32::MIN
                    }
                    if !is_negative && acc > i32::MAX as u64 {
                        return i32::MAX
                    }

                    state = State::AfterSign;
                },
                ('+', State::LeadingSpace) => {
                    is_negative = false;
                    state = State::AfterSign;
                },
                ('-', State::LeadingSpace) => {
                    is_negative = true;
                    state = State::AfterSign;
                },
                (_, _) => break,
            }
        }

        if is_negative {
            let value = -(acc as i64);
            assert!(value >= i32::MIN as i64);
            value as i32
        } else {
            let value = acc;
            assert!(value <= i32::MAX as u64);
            value as i32
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum State {
    LeadingSpace,
    AfterSign,
}
