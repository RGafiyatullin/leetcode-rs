use utils::list::ListNode;
pub struct Solution;

impl Solution {
    #[inline(always)]
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut fw = vec![];
        let mut carry = 0;

        while l1.is_some() || l2.is_some() {
            let mut acc = 0;

            let c = std::mem::take(&mut carry);
            add(&mut acc, &mut carry, c);

            take_and_add(&mut l1, &mut acc, &mut carry);
            take_and_add(&mut l2, &mut acc, &mut carry);

            fw.push(acc);
        }
        if carry != 0 || fw.is_empty() {
            fw.push(std::mem::take(&mut carry));
        }

        let mut out = None;
        while let Some(val) = fw.pop() {
            out = Some(Box::new(ListNode { val, next: out.take() }));
        }
        out
    }
}

const RADIX: i32 = 10;

#[inline(always)]
fn take_and_add(l: &mut Option<Box<ListNode>>, acc: &mut i32, carry: &mut i32) {
    if let Some(mut n) = l.take() {
        *l = n.next.take();
        add(acc, carry, n.val);
    }
}

#[inline(always)]
fn add(acc: &mut i32, carry: &mut i32, addend: i32) {
    let sum = *acc + addend;

    *acc = sum % RADIX as i32;
    *carry += sum / RADIX as i32;
}
