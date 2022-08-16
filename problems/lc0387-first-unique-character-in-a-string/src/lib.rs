pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        const MIN_CHAR: char = 'a';
        const MAX_CHAR: char = 'z';
        const TOTAL_CHAR_COUNT: usize = MAX_CHAR as usize - MIN_CHAR as usize + 1;

        #[derive(Debug, Clone, Copy)]
        enum Counter {
            None,
            First(usize),
            Consequent,
        }

        let mut map = [Counter::None; TOTAL_CHAR_COUNT];

        for (idx, ch) in s.chars().enumerate() {
            let ch = ch as usize - MIN_CHAR as usize;
            map[ch] = match map[ch] {
                Counter::None => Counter::First(idx),
                _ => Counter::Consequent,
            }
        }

        map.iter()
            .copied()
            .filter_map(|c| match c {
                Counter::First(idx) => Some(idx as i32),
                _ => None,
            })
            .min()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&str, i32)] = &[("leetcode", 0), ("loveleetcode", 2), ("aabb", -1)];

    #[test]
    fn run_all_cases() {
        for &(input, exp) in CASES {
            eprintln!("{:?} -> {:?}", input, exp);
            assert_eq!(Solution::first_uniq_char(input.to_owned()), exp);
        }
    }
}
