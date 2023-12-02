pub struct Solution;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        use std::collections::HashMap;

        let chars = chars.chars().fold(HashMap::<_, usize>::new(), |mut acc, ch| {
            *acc.entry(ch).or_default() += 1;
            acc
        });

        words
            .iter()
            .filter(|w| {
                let mut chars = chars.clone();
                for ch in w.chars() {
                    // FUCK YOU LEETCODE! (FFS update the Rust-toolchain already!)
                    if let Some(available) = chars.get_mut(&ch) {
                        if let Some(next_available) = (*available).checked_sub(1) {
                            *available = next_available;
                        } else {
                            return false
                        };
                    } else {
                        return false
                    };
                }
                true
            })
            .map(|s| s.len())
            .sum::<usize>() as _
    }
}
