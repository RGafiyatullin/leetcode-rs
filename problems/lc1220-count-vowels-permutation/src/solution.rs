pub struct Solution;

const MOD_P: usize = 1_000_000_000 + 7;

// Each character is a lower case vowel 'a' (0), 'e' (1), 'i' (2), 'o' (3), 'u' (4).
const RULES: &[&[usize]] = &[
    &[1],          // Each vowel 'a' may only be followed by an 'e'.
    &[0, 2],       // Each vowel 'e' may only be followed by an 'a' or an 'i'.
    &[0, 1, 3, 4], // Each vowel 'i' **may not** be followed by another 'i'.
    &[2, 4],       // Each vowel 'o' may only be followed by an 'i' or a 'u'.
    &[0],          // Each vowel 'u' may only be followed by an 'a'.
];
const VOWELS_COUNT: usize = RULES.len();

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut acc: [usize; VOWELS_COUNT] = [1; VOWELS_COUNT];

        for _ in 2..=n {
            let mut next = [0; VOWELS_COUNT];
            for (from, count) in acc.into_iter().enumerate() {
                for to in RULES[from].into_iter().copied() {
                    next[to] += count % MOD_P;
                    next[to] %= MOD_P;
                }
            }
            acc = next;
        }

        acc.into_iter().fold(0 as usize, |acc, item| (acc + (item % MOD_P)) % MOD_P) as i32
    }
}
