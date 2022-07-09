pub struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let rev_chars = s.chars()
            .rev()
            .filter(|ch| ch.is_alphanumeric())
            .map(|ch| ch.is_ascii_uppercase());
        
        unimplemented!()
    }
}

