pub struct Solution;

const LETTERS_COUNT: usize = 26;

impl Solution {
    pub fn word_subsets(words: Vec<String>, matches: Vec<String>) -> Vec<String> {
        let mut full_digest = [0; LETTERS_COUNT];
        for m in matches {
            let mut m_digest = [0; LETTERS_COUNT];
            word_digest(&mut m_digest, m.as_str());
            update_digest(&mut full_digest, &m_digest);
        }

        words
            .into_iter()
            .filter(|w| check_match(&full_digest, w.as_str()))
            .collect()
    }
}

fn check_match(digest: &[usize; LETTERS_COUNT], word: &str) -> bool {
    let mut w_digest = [0; LETTERS_COUNT];
    word_digest(&mut w_digest, word);

    w_digest
        .iter()
        .copied()
        .zip(digest.into_iter().copied())
        .all(|(w, m): (usize, usize)| w >= m)
}

fn word_digest(digest: &mut [usize; LETTERS_COUNT], word: &str) {
    for ch in word.chars() {
        digest[char_to_idx(ch)] += 1;
    }
}

fn update_digest(acc: &mut [usize; LETTERS_COUNT], item: &[usize; LETTERS_COUNT]) {
    for i in 0..LETTERS_COUNT {
        acc[i] = acc[i].max(item[i]);
    }
}

fn char_to_idx(ch: char) -> usize {
    ch as usize - 'a' as usize
}
