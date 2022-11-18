pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes(); // requirement: s consists of English letters, digits, symbols and spaces.

        let mut max = 0;
        let mut digest = [0usize; u8::MAX as usize];
        let mut head = 0;

        for tail in 0..s.len() {
            let ch = s[tail];
            digest[char_idx(ch)] += 1;

            if digest[char_idx(ch)] > 1 {
                for bump_head in head..tail {
                    let removed = s[bump_head];
                    digest[char_idx(removed)] -= 1;
                    head = bump_head + 1;
                    if removed == ch {
                        break
                    }
                }
            } else {
                max = std::cmp::max(max, tail - head + 1);
            }
        }

        max as i32
    }
}

fn char_idx(ch: u8) -> usize {
    ch as usize
}
