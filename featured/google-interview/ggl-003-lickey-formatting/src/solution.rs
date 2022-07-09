pub struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let k = k as usize;
        let mut out = s
            .chars()
            .rev()
            .filter(|ch| ch.is_alphanumeric())
            .map(|ch| ch.to_ascii_uppercase())
            .enumerate()
            .fold(vec![], |mut acc, (idx, ch)| {
                acc.push(ch);

                if idx % k == k - 1 {
                    acc.push('-');
                }

                acc
            });

        if let Some('-') = out.last() {
            out.pop();
        }
        out.reverse();

        out.into_iter().collect()
    }
}
