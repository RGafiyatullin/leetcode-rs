pub struct Solution;

const RUNES: &[&[u8]] = &[
    b"1", b"2", b"3", b"4", b"5", b"6", b"7", b"8", b"9", b"10", b"11", b"12", b"13", b"14", b"15",
    b"16", b"17", b"18", b"19", b"20", b"21", b"22", b"23", b"24", b"25", b"26",
];

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        use std::collections::HashSet;

        let s = s.as_bytes();

        let runes: HashSet<_> = RUNES.iter().copied().collect();
        let max_rune_len = RUNES.iter().map(|r| r.len()).max().expect("Empty runes list");

        let mut memo = vec![0; s.len() + 1];
        memo[0] = 1;

        for i in 0..s.len() {
            let tail = &s[i..];
            let prev = memo[i];

            for rune_len in 1..=tail.len().min(max_rune_len) {
                let rune_candidate = &tail[0..rune_len];
                if runes.contains(&rune_candidate) {
                    memo[i + rune_len] += prev;
                }
            }
        }
        memo.last().copied().unwrap_or(1) as i32
    }
}
