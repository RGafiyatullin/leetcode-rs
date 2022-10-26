#[test]
fn run_all_cases() {
    for &(input, expected) in CASES {
        eprintln!("{:?} -> {:?}", input, expected);
        assert_eq!(crate::active_vec::Solution::is_possible(input.to_vec()), expected);
        assert_eq!(crate::active_heap::Solution::is_possible(input.to_vec()), expected);
    }
}

pub const CASES: &[(&[i32], bool)] = &[
    (&[1, 2, 3, 3, 4, 5], true),
    (&[1, 2, 3, 3, 4, 4, 5, 5], true),
    (&[1, 2, 3, 4, 4, 5], false),
    (&[1], false),
    (&[1, 2], false),
    (&[1, 1], false),
    (&[1, 1, 1], false),
    (&[1, 2, 3], true),
    (&[1, 1, 2], false),
    (&[1, 2, 3, 5], false),
    (&[1, 2, 3, 5, 6], false),
    (&[1, 2, 3, 5, 6, 7], true),
];
