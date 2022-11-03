pub struct Solution;

const MIN_LETTER: usize = 'a' as usize;
const MAX_LETTER: usize = 'z' as usize;
const RANGE_SIZE: usize = MAX_LETTER - MIN_LETTER + 1;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        // eprintln!("min: {:?}", MIN_LETTER);
        // eprintln!("max: {:?}", MAX_LETTER);
        // eprintln!("range: {:?}", RANGE_SIZE);

        let mut freq_map = [[0usize; RANGE_SIZE]; RANGE_SIZE];

        words
            .into_iter()
            // .inspect(|w| eprintln!("w: {:?}", w))
            .map(|w| word_to_offsets(w.as_bytes()))
            // .inspect(|&(x, y)| eprintln!("[{:?}:{:?}]", x, y))
            .for_each(|(x, y)| {
                freq_map[x][y] += 1;
            });

        let count_regular: usize =
            (0..RANGE_SIZE).flat_map(|x| (0..x).map(move |y| (x, y)))
                .map(|(x,y)| {
                    std::cmp::min(freq_map[x][y], freq_map[y][x]) * 2 /* two words */ * 2 /* two letters each */
                })
                .sum();

        // eprintln!("count-regular: {:?}", count_regular);

        let (quot_sum, rem_sum) = (0..RANGE_SIZE)
            .map(|x| freq_map[x][x])
            .fold((0, 0), |(q, r), c| (q + c / 2, r + c % 2));

        // eprintln!("quot-sum: {:?}", quot_sum);
        // eprintln!("rem-sum: {:?}", rem_sum);

        let out = count_regular +
            quot_sum * 2 /* two words */ * 2 /* two letters each */ +
            std::cmp::min(rem_sum, 1) * 2 /* two letters each */;

        out as i32
    }
}

fn word_to_offsets(word: &[u8]) -> (usize, usize) {
    assert!(word.len() == 2);
    (byte_to_offset(word[0]), byte_to_offset(word[1]))
}

fn byte_to_offset(b: u8) -> usize {
    let b = b as usize;
    assert!(b <= MAX_LETTER);
    assert!(b >= MIN_LETTER);

    b - MIN_LETTER
}
