// https://leetcode.com/problems/word-ladder-ii/

fn main() -> () {
    let stdin = std::io::stdin();

    let mut begin_word = String::new();
    let mut end_word = String::new();
    let mut word_list = String::new();

    stdin.read_line(&mut begin_word).unwrap();
    stdin.read_line(&mut end_word).unwrap();
    stdin.read_line(&mut word_list).unwrap();

    let begin_word = begin_word.trim().to_owned();
    let end_word = end_word.trim().to_owned();
    let word_list = word_list
        .trim()
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();

    let solution = Solution::find_ladders(begin_word, end_word, word_list);

    println!("{:#?}", solution);
}

struct Solution;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut used_words = word_list.iter().map(|_| false).collect::<Vec<_>>();

        if let Some(end_word_idx) = word_list
            .iter()
            .enumerate()
            .find(|(_idx, s)| *s == &end_word)
            .map(|(idx, _s)| idx)
        {
            used_words[end_word_idx] = true;

            let mut layer = vec![end_word_idx];
            
            loop {
                let mut next_layer = Vec::new();
                
                for prev_idx in layer {
                    
                }

                layer = next_layer;
            }
            
        } else {
            return Vec::new();
        }

        unimplemented!()
    }
}

 

struct Heap<I> {
    data: Vec<(Option<usize>, I)>,
}
impl<I> Heap<I> {
    pub fn new(capacity: usize, root: I) -> Self {
        let mut data = Vec::with_capacity(capacity);
        data.push((None, root));
        Heap { data }
    }
    pub fn push(&mut self, parent: usize, item: I) -> usize {
        let idx = self.data.len();
        let tuple = (Some(parent), item);
        self.data.push(tuple);
        idx
    }
    pub fn get(&self, idx: usize) -> impl Iterator<Item = &I> {
        if idx >= self.data.len() {
            AncestryIterator {
                heap: self,
                next: None,
            }
        } else {
            AncestryIterator {
                heap: self,
                next: Some(idx),
            }
        }
    }
}

struct AncestryIterator<'a, I> {
    heap: &'a Heap<I>,
    next: Option<usize>,
}
impl<'a, I> Iterator for AncestryIterator<'a, I> {
    type Item = &'a I;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(idx) => {
                let tuple = &self.heap.data[idx];
                self.next = tuple.0;
                Some(&tuple.1)
            }
        }
    }
}

fn distance_is_1<W1: AsRef<str>, W2: AsRef<str>>(w1: W1, w2: W2) -> bool {
    let mut diff = 0;
    let cs1 = w1.as_ref().chars();
    let cs2 = w2.as_ref().chars();
    for (c1, c2) in cs1.zip(cs2) {
        if c1 == c2 {
            continue;
        } else {
            diff = diff + 1;
            if diff > 1 {
                return false;
            }
        }
    }
    return diff == 1;
}

#[test]
fn distance_is_1_test() {
    let data = vec![
        ("abc", "abc", false),
        ("abc", "abd", true),
        ("abc", "acb", false),
        ("", "", false),
    ];
    for (w1, w2, expected) in data {
        assert_eq!(distance_is_1(w1, w2), expected);
    }
}

#[test]
fn heap_test() {
    let mut heap = Heap::<&'static str>::new(100, "");
    let idx_1 = heap.push(0, "1");
    let idx_2 = heap.push(0, "2");
    let idx_11 = heap.push(idx_1, "11");
    let idx_12 = heap.push(idx_1, "12");
    let idx_21 = heap.push(idx_2, "21");
    let idx_22 = heap.push(idx_2, "22");

    assert_eq!(
        heap.get(idx_21).map(|s| s.to_owned()).collect::<Vec<_>>(),
        vec!["21", "2", ""]
    );
    assert_eq!(
        heap.get(idx_22).map(|s| s.to_owned()).collect::<Vec<_>>(),
        vec!["22", "2", ""]
    );
    assert_eq!(
        heap.get(idx_12).map(|s| s.to_owned()).collect::<Vec<_>>(),
        vec!["12", "1", ""]
    );
    assert_eq!(
        heap.get(idx_11).map(|s| s.to_owned()).collect::<Vec<_>>(),
        vec!["11", "1", ""]
    );
    assert_eq!(
        heap.get(idx_1).map(|s| s.to_owned()).collect::<Vec<_>>(),
        vec!["1", ""]
    );
    assert_eq!(
        heap.get(idx_2).map(|s| s.to_owned()).collect::<Vec<_>>(),
        vec!["2", ""]
    );
    assert_eq!(
        heap.get(0).map(|s| s.to_owned()).collect::<Vec<_>>(),
        vec![""]
    );
    assert_eq!(
        heap.get(idx_22 + 1)
            .map(|s| s.to_owned())
            .collect::<Vec<_>>(),
        Vec::<String>::new()
    );
}
