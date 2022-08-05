use crate::Solution;

const CASES: &[(&[&[i32]], i32)] = &[
    (&[&[1]], 0),
    (
        &[
            &[4, 3, 2, -1],
            &[3, 2, 1, -1],
            &[1, 1, -1, -2],
            &[-1, -1, -2, -3],
        ],
        8,
    ),
    (&[&[3, 2], &[1, 0]], 0),
    (&[&[3, 2, 1, 0, -1, -2]], 2),
    (&[&[3], &[2], &[1], &[0], &[-1], &[-2]], 2),
];

#[test]
fn run_all_cases() {
    for &(m, exp) in CASES {
        eprintln!("{:?} => {:?}", m, exp);
        assert_eq!(
            Solution::count_negatives(m.into_iter().copied().map(<[_]>::to_vec).collect()),
            exp
        );
    }
}
