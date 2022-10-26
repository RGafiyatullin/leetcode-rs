pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        assert!(s.len() >= 1);
        assert!(s.len() <= 10_000);
        assert!(words.len() >= 1);
        assert!(words.len() <= 5_000);

        let s = s.as_bytes();

        let words: Vec<_> = words.iter().map(String::as_bytes).collect();
        let word_len = words
            .iter()
            .copied()
            .map(<[_]>::len)
            .fold(None, |acc, len| {
                assert!(acc.is_none() || acc == Some(len));
                Some(len)
            })
            .expect("words is a non-empty array");

        let word_count: HashMap<&[u8], usize> =
            words.iter().copied().fold(HashMap::with_capacity(words.len()), |mut acc, w| {
                *acc.entry(w).or_default() += 1;
                acc
            });

        let window_size = word_len * words.len();

        if window_size > s.len() {
            vec![]
        } else {
            let mut output = vec![];
            for offset in 0..=(s.len() - window_size) {
                let mut sub_s = &s[offset..][0..window_size];
                let mut word_count = word_count.to_owned();

                while !sub_s.is_empty() {
                    let word = &sub_s[0..word_len];

                    if let Some(c) = word_count.get_mut(word) {
                        *c -= 1;
                        if *c == 0 {
                            word_count.remove(word);
                        }
                        sub_s = &sub_s[word_len..];
                    } else {
                        break
                    }
                }
                if word_count.is_empty() {
                    output.push(offset as i32)
                }
            }

            output
        }
    }
}
