pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let (word_count, trie) = {
            let mut word_count: HashMap<&str, usize> = Default::default();
            let mut trie: Vec<Node> = vec![Default::default()];

            for word in words.iter() {
                assert!(word.len() >= 1);
                assert!(word.len() <= 30);

                let mut node_idx = 0;
                for ch in word.chars() {
                    let ch = char_encode(ch);

                    node_idx = if let Some(next) = trie[node_idx].next[ch] {
                        next
                    } else {
                        let next = trie.len();
                        trie[node_idx].next[ch] = Some(next);
                        trie.push(Default::default());
                        next
                    };
                }
                trie[node_idx].hit = Some(word);
                *word_count.entry(word).or_default() += 1;
            }
            (word_count, trie)
        };

        let mut active_contexts: HashSet<usize> = Default::default();
        let mut all_contexts = vec![];
        let mut complete_contexts = vec![];

        for (idx, ch) in s.chars().enumerate() {
            assert_eq!(idx, all_contexts.len());

            // eprintln!("- [{:?}] = {:?}", idx, ch);

            all_contexts.push(Context::new(&trie[..], &word_count));
            active_contexts.insert(idx);

            let mut evicted_contexts = vec![];
            for active_idx in active_contexts.iter().copied() {
                let ctx = &mut all_contexts[active_idx];
                match ctx.process_char(ch) {
                    ContextState::Working => {
                        // eprintln!("    {:?} — working {:?}", active_idx, ctx.word_count);
                        ()
                    }
                    ContextState::Complete => {
                        // eprintln!("    {:?} — complete! {:?}", active_idx, ctx.word_count);
                        evicted_contexts.push(active_idx);
                        complete_contexts.push(active_idx);
                    }
                    ContextState::Failed => {
                        // eprintln!("    {:?} — failed: {:?}", active_idx, ctx.word_count);
                        evicted_contexts.push(active_idx)
                    }
                }
            }
            for evicted_idx in evicted_contexts {
                assert!(active_contexts.remove(&evicted_idx));
            }
        }

        complete_contexts
            .into_iter()
            .map(|idx| idx as i32)
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
struct Context<'a> {
    current_idx: usize,
    trie: &'a [Node<'a>],
    word_count: HashMap<&'a str, usize>,
}

impl<'a> Context<'a> {
    pub fn new(trie: &'a [Node], word_count: &HashMap<&'a str, usize>) -> Self {
        Self {
            current_idx: 0,
            trie,
            word_count: word_count.to_owned(),
        }
    }

    pub fn process_char(&mut self, ch: char) -> ContextState {
        let ch = char_encode(ch);
        let current_node = &self.trie[self.current_idx];
        if let Some(next_idx) = current_node.next[ch] {
            self.current_idx = next_idx;

            let next_node = &self.trie[next_idx];
            if let Some(hit) = next_node.hit {
                self.current_idx = 0;

                if let Some(wc) = self.word_count.get_mut(&hit) {
                    if let Some(next_wc) = wc.checked_sub(1) {
                        *wc = next_wc;

                        if next_wc == 0 {
                            self.word_count.remove(&hit);
                        }

                        if self.word_count.is_empty() {
                            ContextState::Complete
                        } else {
                            ContextState::Working
                        }
                    } else {
                        ContextState::Failed
                    }
                } else {
                    ContextState::Failed
                }
            } else {
                ContextState::Working
            }
        } else {
            ContextState::Failed
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ContextState {
    Complete,
    Failed,
    Working,
}

const LETTER_MIN: char = 'a';
const LETTER_MAX: char = 'z';
const LETTERS_COUNT: usize = (LETTER_MAX as u8 - LETTER_MIN as u8 + 1) as usize;

fn char_encode(ch: char) -> usize {
    assert!(ch >= LETTER_MIN);
    assert!(ch <= LETTER_MAX);

    (ch as u8 - LETTER_MIN as u8) as usize
}

#[derive(Debug, Clone, Copy, Default)]
struct Node<'a> {
    hit: Option<&'a str>,
    next: [Option<usize>; LETTERS_COUNT],
}
