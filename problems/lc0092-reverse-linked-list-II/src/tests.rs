use crate::{ListNode, Solution};

#[test]
fn case_01() {
    let mut head = Option::None;
    for val in [1, 2, 3, 4, 5].into_iter().rev() {
        let node = ListNode { val, next: head };
        head = Some(Box::new(node));
    }

    assert_eq!(
        Solution::reverse_between(head, 2, 4).unwrap().into_vec(),
        &[1, 4, 3, 2, 5]
    );
}

impl ListNode {
    fn into_vec(self) -> Vec<i32> {
        self.extend_vec(vec![])
    }
    fn extend_vec(self, mut vec: Vec<i32>) -> Vec<i32> {
        vec.push(self.val);
        if let Some(next) = self.next {
            next.extend_vec(vec)
        } else {
            vec
        }
    }
}
