use crate::solution::Solution;

const CASES: &[(&[&[i32]], &[i32])] = &[
    (
        &[
            &[1, 1, 1, -1, -1],
            &[1, 1, 1, -1, -1],
            &[-1, -1, -1, 1, 1],
            &[1, 1, 1, 1, -1],
            &[-1, -1, -1, -1, -1],
        ],
        &[1, -1, -1, -1, -1],
    ),
    (&[&[-1]], &[-1]),
    (
        &[
            &[1, 1, 1, 1, 1, 1],
            &[-1, -1, -1, -1, -1, -1],
            &[1, 1, 1, 1, 1, 1],
            &[-1, -1, -1, -1, -1, -1],
        ],
        &[0, 1, 2, 3, 4, -1],
    ),
];

#[test]
fn all_cases() {
    for &(input, expected) in CASES {
        let input = input.into_iter().copied().map(<[_]>::to_vec).collect::<Vec<_>>();
        assert_eq!(Solution::find_ball(input), expected);
    }
}
