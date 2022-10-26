pub struct Solution;
#[cfg(test)]
mod tests;

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        mut word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        assert!(begin_word.len() >= 1);
        assert!(begin_word.len() <= 5);
        assert_eq!(begin_word.len(), end_word.len());
        assert!(word_list.len() >= 1);
        assert!(word_list.len() <= 500);

        let target_word_idx = word_list.len();
        word_list.push(begin_word);
        let word_list = word_list;

        let (source_word_idx, bridge_to_words, word_to_bridges) = {
            let mut bridge_to_words = HashMap::<Bridge, Vec<usize>>::new();
            let mut word_to_bridges: Vec<Vec<Bridge>> = vec![vec![]; word_list.len()];
            let mut end_word_idx = None;

            for (idx, word) in word_list.iter().enumerate() {
                if word.as_bytes() == end_word.as_bytes() {
                    end_word_idx = Some(idx);
                }

                for bridge in word_bridges(word.as_bytes()) {
                    bridge_to_words.entry(bridge).or_default().push(idx);
                    word_to_bridges[idx].push(bridge);
                }
            }

            if let Some(end_word_idx) = end_word_idx {
                (end_word_idx, bridge_to_words, word_to_bridges)
            } else {
                return vec![]
            }
        };

        let mut memo: Vec<(usize, Vec<usize>)> =
            vec![(word_list.len() + 1, Default::default()); word_list.len()];
        memo[source_word_idx] = (0, Default::default());

        let mut tasks: VecDeque<_> = vec![source_word_idx].into();

        let mut best_path_so_far = Option::<usize>::None;

        while let Some(source_word_idx) = tasks.pop_front() {
            use std::cmp::Ordering::*;

            let (path_len, _) = memo[source_word_idx];

            if best_path_so_far.into_iter().any(|best| path_len >= best) {
                continue
            }

            for bridge in word_to_bridges[source_word_idx].iter() {
                for word_idx in bridge_to_words
                    .get(bridge)
                    .map(AsRef::<[usize]>::as_ref)
                    .unwrap_or(&[])
                    .iter()
                    .copied()
                {
                    if word_list[word_idx] == word_list[source_word_idx] &&
                        word_idx != source_word_idx
                    {
                        continue
                    }

                    let (existing_path_len, from_words) = &mut memo[word_idx];

                    let improvement = match (path_len + 1).cmp(existing_path_len) {
                        Less => {
                            *existing_path_len = path_len + 1;
                            from_words.clear();
                            from_words.push(source_word_idx);
                            true
                        },
                        Equal => {
                            from_words.push(source_word_idx);
                            false
                        },
                        Greater => false,
                    };
                    if word_idx == target_word_idx {
                        best_path_so_far = Some(path_len + 1);
                    } else if improvement {
                        tasks.push_back(word_idx);
                    }
                }
            }
        }

        // eprintln!("BEST-PATH-LEN: {:?}", best_path_so_far);
        // eprintln!("MEMO");
        // for (idx, (len, from_words)) in memo.iter().enumerate() {
        //     eprintln!(
        //         "- [{:?}] {:?} <- {:?} [{:?}]",
        //         idx, word_list[idx], from_words, len
        //     );
        // }

        let mut output = vec![];

        if best_path_so_far.is_some() {
            let mut tasks = vec![(word_list.len() - 1, vec![])];

            while let Some((word_idx, mut path)) = tasks.pop() {
                let (_, next_words) = &memo[word_idx];
                path.push(word_idx);

                // eprintln!(
                //     "... COLLECTING [{:?}]{:?} @ {:?}",
                //     word_idx, word_list[word_idx], path
                // );

                if next_words.is_empty() {
                    output.push(path.into_iter().map(|idx| word_list[idx].to_owned()).collect())
                } else {
                    for next_idx in next_words.iter().copied() {
                        tasks.push((next_idx, path.to_owned()));
                    }
                }
            }
        }
        output
    }
}

fn word_bridges<'a>(word: &'a [u8]) -> impl Iterator<Item = Bridge<'a>> + 'a {
    (0..word.len()).map(move |via| Bridge::new(word, via))
}

#[derive(Debug, Clone, Copy)]
struct Bridge<'a> {
    word: &'a [u8],
    via: usize,
}

impl<'a> Bridge<'a> {
    pub fn new(word: &'a [u8], via: usize) -> Self {
        Self { word, via }
    }
}

impl<'a> PartialEq for Bridge<'a> {
    fn eq(&self, other: &Self) -> bool {
        (&self.word[0..self.via], &self.word[self.via + 1..]) ==
            (&other.word[0..other.via], &other.word[other.via + 1..])
    }
}
impl<'a> Eq for Bridge<'a> {}

impl<'a> Hash for Bridge<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (&self.word[0..self.via], &self.word[self.via + 1..]).hash(state)
    }
}
