pub struct Solution;

use std::collections::HashMap;

const MIN_CHAR: u8 = 'a' as u8;
const MAX_CHAR: u8 = 'z' as u8;
const CHAR_RANGE: usize = (MAX_CHAR - MIN_CHAR) as usize + 1;

// `0 <= strs[i].length <= 100`
// the count of any char cannot exceed 100 ~> u8 is enough
type KEY = [u8; CHAR_RANGE];

impl Solution {
    #[inline(always)]
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .map(|s| (into_key(s.as_bytes()), s))
            .fold(Default::default(), aggregate)
            .into_values()
            .collect()
    }
}

#[inline(always)]
fn aggregate(
    mut acc: HashMap<KEY, Vec<String>>,
    (key, s): (KEY, String),
) -> HashMap<KEY, Vec<String>> {
    acc.entry(key).or_default().push(s);
    acc
}

#[inline(always)]
fn into_key(s: impl AsRef<[u8]>) -> KEY {
    s.as_ref().into_iter().copied().fold(Default::default(), |mut acc, ch| {
        acc[letter_to_idx(ch)] += 1;
        acc
    })
}

#[inline(always)]
fn letter_to_idx(ch: u8) -> usize {
    assert!(MIN_CHAR <= MAX_CHAR);
    assert!(ch >= MIN_CHAR);
    assert!(ch <= MAX_CHAR);

    (ch - MIN_CHAR) as usize
}
