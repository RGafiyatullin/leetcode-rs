pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // s consists only of printable ASCII characters.
        let s = s.as_bytes();

        let map = init_map();

        let head_to_tail = s
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(idx, ch)| map[ch as usize].map(|ch| (idx, ch)));
        let tail_to_head = s
            .iter()
            .copied()
            .enumerate()
            .rev()
            .filter_map(|(idx, ch)| map[ch as usize].map(|ch| (idx, ch)));

        for ((head_idx, head_ch), (tail_idx, tail_ch)) in head_to_tail.zip(tail_to_head) {
            // eprintln!("h: [{}]={}", head_idx, head_ch as char);
            // eprintln!("t: [{}]={}", tail_idx, tail_ch as char);
            if head_idx >= tail_idx {
                break
            }
            if head_ch != tail_ch {
                return false
            }
        }

        true
    }
}

#[inline(always)]
fn init_map() -> [Option<u8>; 256] {
    let mut map = [Option::<u8>::None; 256];

    for ch in ('a'..='z').chain('0'..='9') {
        let ch = ch as u8;
        map[ch as usize] = Some(ch);
    }

    for (ch_low, ch_cap) in ('a'..='z').zip('A'..='Z') {
        let ch_low = ch_low as u8;
        let ch_cap = ch_cap as u8;
        map[ch_cap as usize] = Some(ch_low);
    }

    map
}
