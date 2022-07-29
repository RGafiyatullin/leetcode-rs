pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        normalize(s.as_str()) == normalize(t.as_str())
    }
}

use std::collections::HashMap;
fn normalize(s: &str) -> Vec<u8> {
    let mut next: u8 = 0;
    let mut map = HashMap::<char, u8>::new();   
    let mut out = Vec::<u8>::new();

    for c in s.chars() {
        if let Some(m) = map.get(&c).copied() {
            out.push(m);
        } else {
            out.push(next);
            map.insert(c, next);
            next += 1;
        }
    }

    out
}

#[test]
fn test() {
    for &(left, right, expected) in &[
        ("egg", "add", true),
        ("foo", "bar", false),
        ("paper", "title", true),
    ] {
        assert_eq!(Solution::is_isomorphic(left.to_owned(), right.to_owned()), expected);
    }
}
