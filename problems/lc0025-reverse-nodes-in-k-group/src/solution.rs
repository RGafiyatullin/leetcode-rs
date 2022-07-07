use super::*;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;

        let mut buf = Vec::<i32>::with_capacity(k);
        let mut input: LL = head.into();
        let mut output: LL = Default::default();

        loop {
            buf.extend(input.take(k));
            if buf.len() == k {
                buf.reverse();
                output.extend(buf.drain(..));
            } else {
                assert!(buf.len() < k);
                output.extend(buf.drain(..));
                break reverse(output).into();
            }
        }
    }
}

struct Buf(Vec<i32>);

impl Extend<i32> for Buf {
    fn extend<I: IntoIterator<Item = i32>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

#[derive(Debug, Default)]
struct LL(Option<Box<ListNode>>);

impl<'a> Iterator for &'a mut LL {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            None => None,
            Some(node) => {
                self.0 = node.next;
                Some(node.val)
            }
        }
    }
}

impl Extend<i32> for LL {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = i32>,
    {
        for i in iter {
            self.0 = Some(Box::new(ListNode {
                val: i,
                next: self.0.take(),
            }));
        }
    }
}

impl From<Option<Box<ListNode>>> for LL {
    fn from(head: Option<Box<ListNode>>) -> Self {
        LL(head)
    }
}

impl Into<Option<Box<ListNode>>> for LL {
    fn into(self) -> Option<Box<ListNode>> {
        self.0
    }
}

fn reverse(mut src: LL) -> LL {
    let mut out = LL::default();
    out.extend(&mut src);
    out
}
