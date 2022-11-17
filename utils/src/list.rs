// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl ListNode {
    pub fn from_items(ii: impl IntoIterator<Item = i32>) -> Option<Box<ListNode>> {
        let mut head = None;

        let mut items = ii.into_iter().collect::<Vec<_>>();

        while let Some(val) = items.pop() {
            let cons = ListNode { val, next: head.take() };
            head = Some(Box::new(cons));
        }

        head
    }
}

#[test]
fn build_from_items() {
    let items = &[1, 2, 3];
    let ll = ListNode::from_items(items.into_iter().copied());

    assert_eq!(
        ll,
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None }))
            }))
        }))
    );
}

#[test]
fn build_empty() {
    assert!(ListNode::from_items([]).is_none());
}
