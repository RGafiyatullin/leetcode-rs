#[derive(Clone)]
pub struct WordDictionary(Vec<Node>);

impl WordDictionary {
    pub fn new() -> Self {
        Self(vec![Default::default()])
    }

    pub fn add_word(&mut self, word: String) {
        let nodes = &mut self.0;
        let mut current_idx = 0;

        for ch in word.chars().map(char_to_idx) {
            current_idx = if let Some(next_idx) = nodes[current_idx].children[ch] {
                next_idx
            } else {
                let idx = nodes.len();
                nodes.push(Default::default());
                nodes[current_idx].children[ch] = Some(idx);
                idx
            };
        }

        nodes[current_idx].count += 1;
    }

    pub fn search(&self, word: String) -> bool {
        let nodes = &self.0;
        let word = word.chars().map(char_to_idx_opt);

        let mut tasks = vec![(0, word)];

        while let Some((node_idx, mut letters)) = tasks.pop() {
            let node = &nodes[node_idx];

            match letters.next() {
                Some(None) =>
                    node.children.iter().copied().filter_map(std::convert::identity).for_each(
                        |next_id| {
                            tasks.push((next_id, letters.clone()));
                        },
                    ),
                Some(Some(ch)) =>
                    if let Some(next_id) = node.children[ch] {
                        tasks.push((next_id, letters));
                    },
                None =>
                    if node.count > 0 {
                        return true
                    },
            }
        }

        false
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Node {
    count: usize,
    children: [Option<usize>; LETTERS_COUNT],
}

const LETTER_MIN: u8 = 'a' as u8;
const LETTER_MAX: u8 = 'z' as u8;
const LETTERS_COUNT: usize = (LETTER_MAX - LETTER_MIN + 1) as usize;
fn char_to_idx(ch: char) -> usize {
    char_to_idx_opt(ch).expect("invalid char")
}

fn char_to_idx_opt(ch: char) -> Option<usize> {
    let ch = ch as u8;
    if ch >= LETTER_MIN && ch <= LETTER_MAX {
        Some((ch - LETTER_MIN) as usize)
    } else {
        None
    }
}

fn idx_to_char(idx: usize) -> char {
    (idx as u8 + LETTER_MIN) as char
}

struct FormatNode<'a>(&'a [Node], usize);

use std::fmt;
impl fmt::Debug for WordDictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        FormatNode(self.0.as_ref(), 0).fmt(f)
    }
}

impl fmt::Debug for FormatNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nodes = &self.0;
        let node = &nodes[self.1];

        let mut n = f.debug_map();
        if node.count > 0 {
            n.entry(&"COUNT", &node.count);
        }

        for (idx, next_idx_opt) in node.children.iter().copied().enumerate() {
            if let Some(next_idx) = next_idx_opt {
                let ch = idx_to_char(idx);
                n.entry(&ch, &Self(nodes, next_idx));
            }
        }

        n.finish()
    }
}
