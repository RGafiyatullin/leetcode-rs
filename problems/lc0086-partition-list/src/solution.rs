pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl Solution {
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut lt = Option::<Box<ListNode>>::None;
        let mut lt_tail = &mut lt;

        let mut gte = Option::<Box<ListNode>>::None;
        let mut gte_tail = &mut gte;

        while let Some(mut head_node) = head {
            head = head_node.next.take();

            if head_node.val < x {
                *lt_tail = Some(head_node);
                lt_tail = &mut lt_tail.as_mut().unwrap().next;
            } else {
                *gte_tail = Some(head_node);
                gte_tail = &mut gte_tail.as_mut().unwrap().next;
            }
        }

        *lt_tail = gte;

        lt
    }
}
