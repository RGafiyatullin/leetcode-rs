pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // s: consist of only digits and English letters.
        let s = s.as_bytes();
        let p = longest_palindrome(s);
        std::str::from_utf8(p).expect("bad utf-8?!").to_owned()
    }
}

fn longest_palindrome(input: &[u8]) -> &[u8] {
    let mut best: Option<(usize, usize)> = None;

    for left in 0..input.len() {
        for right in left..input.len() {
            let best_score = if let Some((l, r)) = best { score(l, r) } else { 0 };

            if score(left, right) > best_score && is_palindrome(input, left, right) {
                best = Some((left, right));
            }
        }
    }

    if let Some((l, r)) = best {
        &input[l..=r]
    } else {
        &[]
    }
}

fn score(left: usize, right: usize) -> usize {
    assert!(left <= right);
    right - left + 1
}

fn palindrome_score(input: &[u8], mut left: usize, mut right: usize) -> usize {
    assert!(left < input.len());
    assert!(right < input.len());
    assert!(left <= right);
    let score = score(left, right);

    while left < right {
        if input[left] != input[right] {
            return 0
        }
        left = left.checked_add(1).expect("overflow");
        right = right.checked_sub(1).expect("underflow");
    }

    score
}

fn is_palindrome(input: &[u8], left: usize, right: usize) -> bool {
    palindrome_score(input, left, right) > 0
}

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome(b"babad", 0, 0), true);
    assert_eq!(is_palindrome(b"babad", 0, 1), false);
    assert_eq!(is_palindrome(b"babad", 0, 2), true);
    assert_eq!(is_palindrome(b"babad", 0, 3), false);
    assert_eq!(is_palindrome(b"babad", 0, 4), false);

    assert_eq!(is_palindrome(b"babad", 1, 1), true);
    assert_eq!(is_palindrome(b"babad", 1, 2), false);
    assert_eq!(is_palindrome(b"babad", 1, 3), true);
    assert_eq!(is_palindrome(b"babad", 1, 4), false);

    assert_eq!(is_palindrome(b"cbbd", 0, 0), true);
    assert_eq!(is_palindrome(b"cbbd", 0, 1), false);
    assert_eq!(is_palindrome(b"cbbd", 0, 2), false);
    assert_eq!(is_palindrome(b"cbbd", 0, 3), false);

    assert_eq!(is_palindrome(b"cbbd", 1, 1), true);
    assert_eq!(is_palindrome(b"cbbd", 1, 2), true);
    assert_eq!(is_palindrome(b"cbbd", 1, 3), false);
}
