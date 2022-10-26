pub struct Solution;

fn digit_value(ch: char) -> i32 {
    match ch {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        invalid => panic!("invalid digit: {:?}", invalid),
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        assert!(s.len() <= 15);

        let digits = s.chars().map(digit_value).collect::<Vec<_>>();

        let value = digits.iter().copied().enumerate().fold(0, |acc, (idx, digit)| {
            if digits[idx + 1..].iter().copied().any(|r| r > digit) {
                acc - digit
            } else {
                acc + digit
            }
        });

        value
    }
}
