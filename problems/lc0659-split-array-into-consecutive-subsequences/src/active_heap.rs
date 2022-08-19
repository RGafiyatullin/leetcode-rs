pub struct Solution;

const MIN_SEQ_LEN: usize = 3;

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut active = BinaryHeap::<Seq>::new();
        for n in nums {
            // eprintln!("N: {:?}", n);
            // eprintln!("  ACTIVE: {:?}", active);

            match find_seq_for(&mut active, n) {
                Err { .. } => return false,
                Ok(None) => {
                    active.push(vec![n].into());
                }
                Ok(Some(mut seq)) => {
                    seq.push(n);
                    active.push(seq);
                }
            }
        }
        active.into_iter().all(|s| s.is_long_enough())
    }
}

fn find_seq_for(heap: &mut BinaryHeap<Seq>, n: i32) -> Result<Option<Seq>, ()> {
    let mut kept = vec![];
    let result = loop {
        if let Some(s) = heap.pop() {
            // eprintln!("    CANDIDATE: {:?}", s);

            let last = s.last();
            if last == n - 1 {
                break Ok(Some(s));
            } else if last < n - 1 {
                if s.is_long_enough() {
                    continue;
                } else {
                    break Err(());
                }
            } else {
                assert!(n == last);
                kept.push(s);
                continue;
            }
        } else {
            break Ok(None);
        }
    };
    heap.extend(kept);

    result
}

use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug, Ord, Eq)]
struct Seq(Vec<i32>);

impl Seq {
    pub fn push(&mut self, n: i32) {
        self.0.push(n);
    }
    pub fn is_long_enough(&self) -> bool {
        self.0.len() >= MIN_SEQ_LEN
    }
    pub fn last(&self) -> i32 {
        self.0.last().copied().expect("Expected non-empty sequence")
    }
}

impl From<Vec<i32>> for Seq {
    fn from(inner: Vec<i32>) -> Self {
        assert!(!inner.is_empty());
        Self(inner)
    }
}

impl PartialEq for Seq {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
    }
}

impl PartialOrd for Seq {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.0.len()).partial_cmp(&Reverse(other.0.len()))
    }
}
