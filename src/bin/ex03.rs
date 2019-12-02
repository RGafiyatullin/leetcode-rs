fn main() -> () {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let input = serde_json::from_str::<Vec<i32>>(&input).expect("failed to parse input");
    let input = ListNode::from_vec(input);
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut multilist = MultiList::from(lists);
        let mut acc = Vec::new();
        while let Some(item) = multilist.select() {
            acc.push(item)
        }
        ListNode::from_vec(acc)
    }
}

impl ListNode {
    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut vec = vec;
        vec.reverse();

        let mut acc: Option<Box<ListNode>> = None;
        for item in vec {
            let tail = acc;
            let head = ListNode::new(item);
            let head = ListNode { next: tail, ..head };
            let head = Box::new(head);
            acc = Some(head);
        }

        acc
    }
}

struct MultiList {
    heads: Vec<Option<Box<ListNode>>>,
    index: HashSet<usize>,
}

impl From<Vec<Option<Box<ListNode>>>> for MultiList {
    fn from(heads: Vec<Option<Box<ListNode>>>) -> Self {
        let index = heads
            .iter()
            .enumerate()
            .filter_map(|(idx, l)| l.as_ref().map(|_| idx))
            .collect();
        MultiList { heads, index }
    }
}

impl MultiList {
    fn select(&mut self) -> Option<i32> {
        let mut candidate = None;

        for idx in self.index.iter() {
            let idx = *idx;
            match (peek(self.nth_ref(idx)), candidate) {
                (None, _) => unreachable!("A ref to an empty LList in the index"),
                (Some(item), None) => candidate = Some((idx, item)),
                (Some(item), Some((_idx, candidate_value))) => {
                    if candidate_value < item {
                        continue;
                    } else {
                        candidate = Some((idx, item))
                    }
                }
            }
        }

        match candidate {
            None => None,
            Some((idx, value)) => {
                self.nth_pop(idx);
                Some(value)
            }
        }
    }
    fn nth_ref(&self, n: usize) -> Option<&ListNode> {
        self.heads[n].as_ref().map(|b| b.as_ref())
    }
    fn nth_pop(&mut self, n: usize) {
        match self.heads[n].take() {
            None => (),
            Some(head) => {
                if head.next.is_none() {
                    self.index.remove(&n);
                }
                self.heads[n] = head.next;
            }
        }
    }
}

fn peek(head: Option<&ListNode>) -> Option<i32> {
    head.map(|n| n.val)
}
