pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl Solution {
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let left = left as usize;
        let right = right as usize;

        if left != right {
            assert!(right > left);

            let mut left_cut = &mut head;

            for _ in 1..left {
                left_cut = &mut left_cut.as_mut().expect("left points out of bounds").next;
            }

            let mut tail = left_cut.take();
            let mut stack_top = Option::<Box<ListNode>>::None;

            for _ in left..=right {
                let mut taken = tail.take();
                let taken_mut = taken.as_mut().unwrap();

                tail = taken_mut.next.take();
                taken_mut.next = stack_top;
                stack_top = taken;
            }

            let mut right_cut = &mut stack_top;
            while right_cut.is_some() {
                right_cut = &mut right_cut.as_mut().unwrap().next;
            }
            *right_cut = tail;
            *left_cut = stack_top;
        }

        head
    }
}
