use crate::solution::*;

const CASES: &[(&[&[i32]], i32, bool)] = &[
    (
        &[
            &[1, 4, 7, 11, 15],
            &[2, 5, 8, 12, 19],
            &[3, 6, 9, 16, 22],
            &[10, 13, 14, 17, 24],
            &[18, 21, 23, 26, 30],
        ],
        5,
        true,
    ),
    (
        &[
            &[1, 4, 7, 11, 15],
            &[2, 5, 8, 12, 19],
            &[3, 6, 9, 16, 22],
            &[10, 13, 14, 17, 24],
            &[18, 21, 23, 26, 30],
        ],
        20,
        false,
    ),
];

#[test]
fn run_all_cases() {
    for &(m, tgt, exp) in CASES {
        let res = Solution::search_matrix(m.into_iter().copied().map(<[_]>::to_vec).collect(), tgt);
        assert_eq!(res, exp, "m: {:?}, tgt: {}", m, tgt);
    }
}
