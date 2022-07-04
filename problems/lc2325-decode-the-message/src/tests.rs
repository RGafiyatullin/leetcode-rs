const CASES: &[(&str, &str, &str)] = &[
    (
        "eljuxhpwnyrdgtqkviszcfmabo",
        "zwx hnfx lqantp mnoeius ycgk vcnjrdb",
        "the five boxing wizards jump quickly",
    ),
    (
        "the quick brown fox jumps over the lazy dog",
        "vkbs bs t suepuv",
        "this is a secret",
    ),
];

#[test]
fn test_all() {
    for &(key, message, output) in CASES {
        assert_eq!(
            crate::solution::Solution::decode_message(key.to_owned(), message.to_owned()),
            output.to_owned()
        );
    }
}
