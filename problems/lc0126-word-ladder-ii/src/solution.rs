pub struct Solution;
#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::rc::Rc;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        assert!(begin_word.len() >= 1);
        assert!(begin_word.len() <= 5);
        assert_eq!(begin_word.len(), end_word.len());
        assert!(word_list.len() >= 1);
        assert!(word_list.len() <= 500);

        let (end_word_idx, bridge_to_words, word_to_bridges) = {
            let mut bridge_to_words = HashMap::<Bridge, Vec<usize>>::new();
            let mut word_to_bridges = HashMap::<usize, Vec<Bridge>>::new();
            let mut end_word_idx = None;

            for (idx, word) in word_list.iter().enumerate() {
                if word.as_bytes() == end_word.as_bytes() {
                    end_word_idx = Some(idx);
                }

                for bridge in word_bridges(word.as_bytes()) {
                    bridge_to_words.entry(bridge).or_default().push(idx);
                    word_to_bridges.entry(idx).or_default().push(bridge);
                }
            }

            if let Some(end_word_idx) = end_word_idx {
                (end_word_idx, bridge_to_words, word_to_bridges)
            } else {
                return vec![];
            }
        };

        let mut shortest_path_len = Option::<usize>::None;
        let mut tasks = VecDeque::<Task>::new();
        let mut visited_words = HashMap::<usize, usize>::new();
        let mut output = Vec::<Rc<Path>>::new();

        let path_nil = Rc::new(Path::Nil);
        for bridge in word_bridges(begin_word.as_bytes()) {
            tasks.extend(tasks_for_bridge(
                bridge,
                &mut visited_words,
                &bridge_to_words,
                1,
                Rc::clone(&path_nil),
            ));
        }

        while let Some(Task {
            word_idx,
            path_len,
            path,
        }) = tasks.pop_front()
        {
            if matches!(shortest_path_len, Some(shortest_path_len) if shortest_path_len < path_len)
            {
                continue;
            }

            if word_idx == end_word_idx {
                assert!(shortest_path_len.is_none() || shortest_path_len == Some(path_len));
                shortest_path_len = Some(path_len);

                output.push(path);
                continue;
            }

            for bridge in word_to_bridges
                .get(&word_idx)
                .map(AsRef::<[Bridge]>::as_ref)
                .unwrap_or(&[])
                .iter()
                .copied()
            {
                tasks.extend(tasks_for_bridge(
                    bridge,
                    &mut visited_words,
                    &bridge_to_words,
                    path_len,
                    Rc::clone(&path),
                ))
            }
        }

        output
            .into_iter()
            .map(|mut path| {
                let mut words = vec![begin_word.to_owned()];

                loop {
                    match &*path {
                        Path::Cons(idx, tail) => {
                            words.push(word_list[*idx].to_owned());
                            path = Rc::clone(tail);
                        }
                        Path::Nil => break,
                    }
                }

                words[1..].reverse();

                // eprintln!("> {:?}", words);
                words
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
}

fn word_bridges<'a>(word: &'a [u8]) -> impl Iterator<Item = Bridge<'a>> + 'a {
    (0..word.len()).map(move |via| Bridge::new(word, via))
}

fn tasks_for_bridge<'a, 'b>(
    bridge: Bridge<'a>,
    visited_words: &'a mut HashMap<usize, usize>,
    bridge_to_words: &'a HashMap<Bridge, Vec<usize>>,
    path_len: usize,
    path: Rc<Path>,
) -> impl Iterator<Item = Task> + 'a {
    bridge_to_words
        .get(&bridge)
        .map(AsRef::<[usize]>::as_ref)
        .unwrap_or(&[])
        .into_iter()
        .copied()
        .filter(move |word_idx| {
            if let Some(prev_path_len) = visited_words.get(word_idx) {
                if *prev_path_len < path_len {
                    false
                } else {
                    visited_words.insert(*word_idx, path_len);
                    true
                }
            } else {
                visited_words.insert(*word_idx, path_len);
                true
            }
        })
        .map({
            let path = Rc::clone(&path);
            move |word_idx| Task {
                word_idx,
                path_len: path_len + 1,
                path: Rc::new(Path::Cons(word_idx, Rc::clone(&path))),
            }
        })
        .inspect(|task| {
            // std::thread::sleep(std::time::Duration::from_millis(100));
            // eprintln!("ADDING TASK: {:?}", task)
        })
}

#[derive(Debug, Clone)]
struct Task {
    word_idx: usize,
    path_len: usize,
    path: Rc<Path>,
}

#[derive(Debug, Clone)]
enum Path {
    Nil,
    Cons(usize, Rc<Self>),
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
        self.word.len() == other.word.len()
            && self
                .word
                .iter()
                .zip(other.word)
                .enumerate()
                .all(|(idx, (l, r))| idx == self.via || *l == *r)
    }
}
impl<'a> Eq for Bridge<'a> {}

impl<'a> Hash for Bridge<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.word.iter().enumerate().for_each(|(idx, letter)| {
            if idx != self.via {
                state.write_usize(idx);
                state.write_u8(*letter);
            }
        });
    }
}
