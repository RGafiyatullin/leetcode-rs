pub struct Solution;

const MOD: u64 = 1_000_000_000 + 7;

const DIGITS_COUNT: usize = 10;

const MOVES: [&[usize]; DIGITS_COUNT] = [
    // 0
    &[4, 6],
    // 1
    &[6, 8],
    // 2
    &[7, 9],
    // 3
    &[4, 8],
    // 4
    &[0, 3, 9],
    // 5
    &[],
    // 6
    &[0, 1, 7],
    // 7
    &[2, 6],
    // 8
    &[1, 3],
    // 9
    &[2, 4],
];

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        assert!(n >= 1);
        assert!(n <= 5000);

        let mut scores = [1u64; DIGITS_COUNT];

        eprintln!("IN:  {:?}", scores);
        for _ in 1..n {
            scores = step(scores);
            eprintln!("***: {:?}", scores);
        }
        eprintln!("OUT: {:?}", scores);

        let result = scores.iter().copied().sum::<u64>() % MOD;
        result as i32
    }
}

pub fn step(input: [u64; DIGITS_COUNT]) -> [u64; DIGITS_COUNT] {
    let mut output = [0; DIGITS_COUNT];

    for this in 0..DIGITS_COUNT {
        for &prev in MOVES[this] {
            output[this] += input[prev];
            output[this] %= MOD;
        }
    }

    output
}
