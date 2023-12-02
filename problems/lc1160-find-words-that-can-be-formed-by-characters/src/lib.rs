pub struct Solution;

const MIN_CHAR: u8 = b'a';
const MAX_CHAR: u8 = b'z';
type Map = [usize; (MAX_CHAR - MIN_CHAR + 1) as usize];


impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let chars = chars.bytes().fold(Map::default(), |mut map, ch| {map[(ch - MIN_CHAR) as usize] += 1; map});

        words
            .iter()
            .filter(|w| {
                let mut chars = chars.clone();
                for ch in w.bytes() {
                    // FUCK YOU LEETCODE! (FFS update the Rust-toolchain already!)
                    if let Some(left) = chars[(ch - MIN_CHAR) as usize].checked_sub(1) {
                        chars[(ch - MIN_CHAR) as usize] = left;
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
