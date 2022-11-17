use utils::list::ListNode;

const CASES: &[(&[i32], &[i32], &[i32])] = &[
    (&[2, 4, 3], &[5, 6, 4], &[7, 0, 8]),
    (&[0], &[0], &[0]),
    (&[9, 9, 9, 9, 9, 9, 9], &[9, 9, 9, 9], &[8, 9, 9, 9, 0, 0, 0, 1]),
];

#[test]
fn run_all_cases() {
    for &(a, b, exp) in CASES {
        let a = ListNode::from_items(a.into_iter().copied());
        let b = ListNode::from_items(b.into_iter().copied());
        let exp = ListNode::from_items(exp.into_iter().copied());

        assert_eq!(crate::solution::Solution::add_two_numbers(a, b), exp);
    }
}
