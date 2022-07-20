pub struct Solution;

use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let index = s.chars().enumerate().fold(
            HashMap::<char, BTreeSet<usize>>::new(),
            |mut index, (idx, ch)| {
                index.entry(ch).or_default().insert(idx);
                index
            },
        );

        words
            .iter()
            .filter(|w| is_match(&index, w.as_str()))
            .count() as i32
    }
}

fn is_match(index: &HashMap<char, BTreeSet<usize>>, word: &str) -> bool {
    let mut min_pos = 0;
    for ch in word.chars() {
        if let Some(positions) = index.get(&ch) {
            if let Some(pos) = positions.range(min_pos..).next() {
                min_pos = *pos + 1;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    return true;
}
