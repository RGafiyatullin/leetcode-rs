pub const CASES: &[(&str, &str, &[i32])] = &[
    ("abc", "abc", &[0]),
    ("abc", "ababc", &[0, 2]),
    ("abca", "aabcaca", &[3, 0, 1]),
    // ("abc", "abcbabc", &[0,4,2]),
    ("aye", "eyeye", &[]),
];

#[test]
fn run_all_cases() {
    for &(stamp, target, exp) in CASES {
        eprintln!("{:?} | {:?} -> {:?}", stamp, target, exp);
        assert_eq!(
            crate::solution::Solution::moves_to_stamp(stamp.to_string(), target.to_string()),
            exp
        );
    }
}
