pub struct Solution;

#[cfg(feature = "naive")]
impl Solution {
    pub fn next_greatest_letter(letters: impl AsRef<[char]>, target: char) -> char {
        let letters = letters.as_ref();

        assert!(letters.len() >= 2);
        assert!(letters.len() <= 10000);

        letters.into_iter().copied().find(|&l| l > target).unwrap_or(letters[0])
    }
}
#[cfg(feature = "bin-search")]
impl Solution {
    pub fn next_greatest_letter(letters: impl AsRef<[char]>, target: char) -> char {
        use std::cmp::Ordering::*;

        let letters = letters.as_ref();

        assert!(letters.len() >= 2);
        assert!(letters.len() <= 10000);

        let (mut lo, mut hi) = (0, letters.len() - 1);
        let idx = loop {
            let idx = (lo + hi) / 2;
            let first = letters[idx];
            let second = letters.get(idx + 1);

            // eprintln!("{:?}->{:?}->{:?} [{:?}, {:?}]", lo, idx, hi, first, second);

            match (lo == hi, first.cmp(&target), second.map(|l| l.cmp(&target))) {
                (true, Greater, _) => break idx,
                (_, Less | Equal, None) => break 0,
                (_, Less | Equal, Some(Greater)) => break idx + 1,
                (_, Less | Equal, Some(Less | Equal)) => lo = idx + 1,
                (false, Greater, _) => hi = idx,
            }
        };

        letters[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&[char], char, char)] = &[
        (&['c', 'f', 'j'], 'a', 'c'),
        (&['c', 'f', 'j'], 'c', 'f'),
        (&['c', 'f', 'j'], 'd', 'f'),
        (&['a', 'b'], 'z', 'a'),
    ];

    #[test]
    fn run_all_cases() {
        for &(letters, target, expected) in CASES {
            eprintln!("{:?} | {:?} => {:?}", letters, target, expected);
            assert_eq!(Solution::next_greatest_letter(letters, target), expected);
        }
    }
}
