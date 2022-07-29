pub use byte_freqs::Solution;

pub mod sort_chars {
    pub struct Solution;

    impl Solution {
        pub fn is_anagram(s: String, t: String) -> bool {
            if s.len() != t.len() {
                return false;
            }

            let mut s = s.chars().collect::<Vec<_>>();
            let mut t = t.chars().collect::<Vec<_>>();

            s.sort();
            t.sort();

            s.into_iter().zip(t).all(|(left, right)| left == right)
        }
    }
}

pub mod sort_bytes {
    pub struct Solution;

    impl Solution {
        pub fn is_anagram(s: String, t: String) -> bool {
            let mut s = s.into_bytes();
            let mut t = t.into_bytes();

            if s.len() != t.len() {
                return false;
            }

            s.sort();
            t.sort();

            s.into_iter().zip(t).all(|(left, right)| left == right)
        }
    }
}

pub mod byte_freqs {
    pub struct Solution;

    impl Solution {
        pub fn is_anagram(s: String, t: String) -> bool {
            assert_eq!(u8::MIN, 0);

            let mut s_freqs = [0 as usize; u8::MAX as usize];
            let mut t_freqs = [0 as usize; u8::MAX as usize];

            for b in s.into_bytes() {
                s_freqs[b as usize] += 1;
            }

            for b in t.into_bytes() {
                t_freqs[b as usize] += 1;
            }

            s_freqs == t_freqs
        }
    }
}

#[test]
fn test() {
    for &(left, right, expected) in &[("", "", true), ("anagram", "nagaram", true)] {
        assert_eq!(
            Solution::is_anagram(left.to_owned(), right.to_owned()),
            expected
        );
    }
}
