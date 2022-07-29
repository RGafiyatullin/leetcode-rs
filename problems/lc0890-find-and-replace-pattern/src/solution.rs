pub struct Solution;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let pattern = normalize(pattern.as_str());
        words
            .into_iter()
            .filter(|w| normalize(w) == pattern)
            .collect()
    }
}

use std::collections::HashMap;

fn normalize(s: &str) -> Vec<u8> {
    let mut next: u8 = 0;
    let mut map = HashMap::<char, u8>::new();
    let mut out = Vec::<u8>::with_capacity(s.len());

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
