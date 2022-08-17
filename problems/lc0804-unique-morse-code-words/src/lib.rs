pub struct Solution;

use std::collections::HashSet;

const CODES: &[&str] = &[
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
    "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
];

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        words
            .iter()
            .map(|w| {
                w.chars()
                    .map(char_to_idx)
                    .flat_map(|idx| CODES[idx].chars())
                    .collect::<String>()
            })
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

fn char_to_idx(ch: char) -> usize {
    let idx = ch as usize - 'a' as usize;
    assert!(idx <= 26);
    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&[&str], i32)] = &[(&["gin", "zen", "gig", "msg"], 2), (&["a"], 1)];

    #[test]
    fn run_all_cases() {
        for &(input, exp) in CASES {
            eprintln!("{:?} -> {:?}", input, exp);
            assert_eq!(
                Solution::unique_morse_representations(
                    input.into_iter().copied().map(ToOwned::to_owned).collect()
                ),
                exp
            );
        }
    }
}
